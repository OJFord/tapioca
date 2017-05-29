use ::inflector::Inflector;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::TokensResult;

fn ident(param: &str) -> Ident {
    Ident::new(param.to_snake_case())
}

fn infer_type(schema: &Yaml) -> TokensResult {
    match schema["type"].as_str() {
        None => Err(From::from("Parameter schema type must be a string.")),
        Some("integer") => {
            match schema["format"].as_str() {
                None => Err(From::from("Parameter schema format must be a string.")),
                Some("int32") => Ok(quote!{i32}),
                Some("int64") => Ok(quote!{i64}),
                Some(_) => Err(From::from("Invalid format for `integer` type.")),
            }
        },
        Some("number") => {
            match schema["format"].as_str() {
                None => Err(From::from("Parameter schema format must be a string.")),
                Some("float") => Ok(quote!{f32}),
                Some("double") => Ok(quote!{f64}),
                Some(_) => Err(From::from("Invalid format for `number` type.")),
            }
        },
        Some("string") => {
            match schema["format"].as_str() {
                None => Ok(quote!{String}),
                Some("byte") => Ok(quote!{tapioca::Base64}),
                Some("binary") => Ok(quote!{&[u8]}),
                Some("date") => Ok(quote!{tapioca::Date}),
                Some("date-time") => Ok(quote!{tapioca::DateTime}),
                Some("password") => Ok(quote!{String}),
                Some(_) => Ok(quote!{String}),
            }
        },
        Some("boolean") => {
            match schema["format"].as_str() {
                None => Ok(quote!{bool}),
                Some(_) => Err(From::from("Unexpected format for `boolean` type.")),
            }
        },
        Some(ptype) => Err(From::from(format!("Parameter type `{}` invalid", ptype))),
    }
}

pub(super) fn infer_v3(schema: &Yaml) -> TokensResult {
    let ident = ident(schema["name"]
        .as_str().expect("Parameter name must be a string.")
    );
    let type_tt = infer_type(&schema["schema"])?;

    Ok(quote!{#ident: #type_tt})
}
