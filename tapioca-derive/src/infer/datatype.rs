use ::std::collections::hash_map::DefaultHasher;
use ::std::hash::{Hash, Hasher};
use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::Error;

type TypeAndNecessaryImpl = Result<
    (Tokens, bool, Option<Tokens>),
    Box<Error + Send + Sync>
>;

pub(super) fn infer_v3(schema: &Yaml) -> TypeAndNecessaryImpl {
    if let Some(schema_ref) = schema["$ref"].as_str() {
        let ref_name = schema_ref.rsplit('/')
            .next().expect("Malformed $ref")
            .to_class_case();
        let ident = Ident::new(ref_name);

        Ok((quote!{ schema_ref::#ident }, false, None))
    } else {
        match schema["type"].as_str() {
            None => Err(From::from("Parameter schema type must be a string.")),

            Some("array") => {
                let (item_type, has_lifetime, supp_types) = infer_v3(&schema["items"])?;
                Ok((quote!{ Vec<#item_type> }, has_lifetime, supp_types))
            },

            Some("object") => {
                let mut fields: Vec<Tokens> = Vec::new();
                let mut additional_types: Vec<Tokens> = Vec::new();
                let required: Vec<&str> = match schema["required"].as_vec() {
                    Some(v) => v.iter()
                        .map(|e| e.as_str()
                            .expect("Required field names must be strings.")
                        )
                        .collect(),
                    None => Vec::new(),
                };
                let mut lifetime = quote!();

                for (name, schema) in schema["properties"].as_hash()
                    .expect("Properties must be a map.")
                {
                    let name = name.as_str()
                        .expect("Property keys must be strings.");

                    let rusty_ident = Ident::new(name.to_snake_case());
                    let (field_type, has_lifetime, supp_types) = infer_v3(&schema)?;

                    if has_lifetime {
                        lifetime = quote!('a);
                    }

                    if let Some(supp_types) = supp_types {
                        additional_types.push(supp_types);
                    }

                    if required.contains(&name) {
                        fields.push(quote!{
                            #[serde(rename=#name)]
                            #rusty_ident: #field_type
                        });
                    } else {
                        fields.push(quote!{
                            #[serde(rename=#name)]
                            #rusty_ident: Option<#field_type>
                        });
                    }
                }

                let mut hasher = DefaultHasher::new();
                let field_strs: Vec<String> = fields.iter()
                    .map(|f| f.to_string())
                    .collect();
                field_strs.hash(&mut hasher);
                let ident = Ident::new(format!("Type{}", hasher.finish()));

                Ok((
                    quote!{ #ident },
                    lifetime == quote!('a),
                    Some(quote!{
                        #(#additional_types)*

                        #[derive(Deserialize)]
                        struct #ident<#lifetime> {
                            #(#fields),*
                        }
                    })
                ))
            },

            Some("integer") => {
                match schema["format"].as_str() {
                    None
                        | Some("int64") => Ok((quote!{i64}, false, None)),
                    Some("int32") => Ok((quote!{i32}, false, None)),
                    Some(_) => Err(From::from("Invalid format for `integer` type.")),
                }
            },

            Some("number") => {
                match schema["format"].as_str() {
                    None
                        | Some("double") => Ok((quote!{f64}, false, None)),
                    Some("float") => Ok((quote!{f32}, false, None)),
                    Some(_) => Err(From::from("Invalid format for `number` type.")),
                }
            },

            Some("string") => {
                match schema["format"].as_str() {
                    None => Ok((quote!{&'a str}, true, None)),
                    Some("byte") => Ok((quote!{::tapioca::Base64}, false, None)),
                    Some("binary") => Ok((quote!{&'a [u8]}, true, None)),
                    Some("date") => Ok((quote!{::tapioca::Date}, false, None)),
                    Some("date-time") => Ok((quote!{::tapioca::DateTime}, false, None)),
                    Some("password") => Ok((quote!{&'a str}, true, None)),
                    Some(_) => Ok((quote!{&'a str}, true, None)),
                }
            },

            Some("boolean") => {
                match schema["format"].as_str() {
                    None => Ok((quote!{bool}, false, None)),
                    Some(_) => Err(From::from("Unexpected format for `boolean` type.")),
                }
            },

            Some(ptype) => Err(From::from(format!("Parameter type `{}` invalid", ptype))),
        }
    }
}
