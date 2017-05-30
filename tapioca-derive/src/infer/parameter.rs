use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::TokensResult;
use infer::FourTokensResult;

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

pub(super) fn infer_v3(struct_ident: &Ident, schema: &Yaml) -> FourTokensResult {
    let mut idents: Vec<Ident> = Vec::new();
    let mut types: Vec<Tokens> = Vec::new();
    let mut name_strs: Vec<Tokens> = Vec::new();
    let mut accessors: Vec<Tokens> = Vec::new();

    for param_schema in schema.as_vec().unwrap() {
        let name = param_schema["name"].as_str()
            .expect("Parameter name must be a string.");
        let field = ident(name);

        idents.push(ident(name));
        types.push(infer_type(&param_schema["schema"])?);
        name_strs.push(quote!{ #name });
        accessors.push(quote!{ query_parameters.#field });
    }

    Ok((
        quote! {
            pub(in super::super) struct #struct_ident {
                #(pub(in super::super) #idents: #types),*
            }
        },
        quote! {},
        quote! { query_parameters: &#struct_ident },
        quote! {
            url.query_pairs_mut()
                .clear()
                #(.append_pair(
                    #name_strs,
                    #accessors.to_string().as_str()
                ))*
                ;
        }
    ))
}
