use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::datatype;
use infer::path;
use infer::TokensResult;

type FieldsAndSupportingTypes = (Vec<Tokens>, Vec<Tokens>);

fn infer_ref(schema: &Yaml) -> FieldsAndSupportingTypes {
    let mut fields: Vec<Tokens> = Vec::new();
    let mut additional_types: Vec<Tokens> = Vec::new();

    for (field_name, schema) in schema["properties"].as_hash()
        .expect("Properties must be a map.")
    {
        let field_name = field_name.as_str()
            .expect("Property must be a string.");
        let field_ident = Ident::new(field_name);
        let (field_type, additional_type) = datatype::infer_v3(&schema).unwrap();

        fields.push(quote!{ #field_ident: #field_type });
        if let Some(additional_type) = additional_type {
            additional_types.push(additional_type);
        }
    }

    (fields, additional_types)
}

pub(super) fn infer_v3(schema: &Yaml) -> TokensResult {
    let paths = schema["paths"].clone();
    let path_impls: Vec<Tokens> = paths.as_hash()
        .expect("Paths must be a map.")
        .iter()
        .map(|(path, path_schema)| path::infer_v3(
            path.as_str().expect("Path must be a string."), &path_schema
        ).unwrap())
        .collect();

    let api_url = schema["servers"][0]["url"].as_str()
        .expect("Must have at least one server URL.");

    let mut schema_ref_struct_defs: Vec<Tokens> = Vec::new();
    for (schema_ref, schema) in schema["components"]["schemas"].as_hash()
        .expect("#/components/schemas must be a map.")
    {
        let schema_ref_name = schema_ref.as_str()
            .expect("$ref name must be a string.");
        let ident = Ident::new(schema_ref_name);
        let (fields, additional_types) = infer_ref(schema);

        schema_ref_struct_defs.push(quote! {
            #(#additional_types)*

            struct #ident {
                #(#fields),*
            }
        });
    }

    Ok(quote! {
        #[macro_use]
        extern crate serde_derive;

        use tapioca::serde as serde;
        use tapioca::serde::json as serde_json;

        mod schema_ref {
            #(#schema_ref_struct_defs)*
        }

        const API_URL: &'static str = #api_url;

        #(#path_impls)*
    })
}
