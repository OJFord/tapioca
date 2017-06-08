use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::datatype;
use infer::path;
use infer::TokensResult;

type FieldsSupport = (Vec<Tokens>, Vec<Tokens>);

fn infer_ref_obj(schema: &Yaml, required: &Vec<Yaml>) -> FieldsSupport {
    let mut fields: Vec<Tokens> = Vec::new();
    let mut additional_types: Vec<Tokens> = Vec::new();

    for (field, schema) in schema["properties"].as_hash()
        .expect("Properties must be a map.")
    {
        let field_name = field.as_str()
            .expect("Property must be a string.");
        let field_ident = Ident::new(field_name);
        let (ty, maybe_at) = datatype::infer_v3(&schema).unwrap();
        let mandate: Tokens;

        if let Some(true) = schema["required"].as_bool() {
            mandate = quote!(::tapioca::datatype::Required);
        } else if required.contains(field) {
            mandate = quote!(::tapioca::datatype::Required);
        } else {
            mandate = quote!(::tapioca::datatype::Optional);
        }

        fields.push(quote!{ #field_ident: #mandate<#ty> });

        if let Some(additional_type) = maybe_at {
            additional_types.push(additional_type);
        }
    }

    (fields, additional_types)
}

fn infer_ref(ident: &Ident, schema: &Yaml, required: &Vec<Yaml>) -> TokensResult {
    match schema["properties"].as_hash() {
        Some(_) => {
            let (fields, additionals) = infer_ref_obj(&schema, &required);

            Ok(quote! {
                #(#additionals)*

                #[derive(Clone, Debug, Deserialize)]
                pub struct #ident {
                    #(pub #fields),*
                }
            })
        },
        None => {
            let (alias_to, maybe_at) = datatype::infer_v3(&schema)?;
            let additional_type = match maybe_at {
                Some(at) => at,
                None => quote!(),
            };

            Ok(quote! {
                #additional_type

                #[allow(dead_code)]
                pub type #ident = #alias_to;
            })
        },
    }
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

    let mut schema_ref_defs: Vec<Tokens> = Vec::new();
    for (schema_ref, schema) in schema["components"]["schemas"].as_hash()
        .expect("#/components/schemas must be a map.")
    {
        let schema_ref_name = schema_ref.as_str()
            .expect("$ref name must be a string.");
        schema_ref_defs.push(infer_ref(
                &Ident::new(schema_ref_name),
                &schema,
                &schema["required"].as_vec().unwrap_or(&Vec::new())
        )?);
    }

    Ok(quote! {
        mod schema_ref {
            use super::schema_ref;

            #(#schema_ref_defs)*
        }

        const API_URL: &'static str = #api_url;

        #(#path_impls)*
    })
}
