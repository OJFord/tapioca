use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::datatype;
use infer::TokensResult;

const UNKNOWN_ERR_CODE: u16 = 520;

fn parse_response_key(key: &Yaml) -> (u16, String) {
    match key.as_i64() {
        Some(code) => (code as u16, format!("Status{}", code)),
        None => (UNKNOWN_ERR_CODE, format!("Status{}", UNKNOWN_ERR_CODE)),
    }
}

pub(super) fn infer_v3(enum_idents: &(Ident, Ident), schema: &Yaml) -> TokensResult {
    let (ref success_en, ref error_en) = *enum_idents;
    let mut error_variants: Vec<Ident> = Vec::new();
    let mut error_models: Vec<Tokens> = Vec::new();
    let mut success_variants: Vec<Ident> = Vec::new();
    let mut success_models: Vec<Tokens> = Vec::new();
    let mut additional_types: Vec<Tokens> = Vec::new();

    for (code, schema) in schema.as_hash()
        .expect("Responses must be a map.")
    {
        let schema = &schema["content"]["application/json"]["schema"];
        schema.as_hash().expect("Only application/json responses are supported.");

        let (status_code, status_str) = parse_response_key(&code);
        let variant_ident = Ident::new(status_str);
        let (inferred_type, additional_type) = datatype::infer_v3(&schema)?;

        if let Some(t) = additional_type {
            additional_types.push(t);
        }

        if status_code < 400 {
            success_variants.push(variant_ident);
            success_models.push(inferred_type);
        } else {
            error_variants.push(variant_ident);
            error_models.push(inferred_type);
        }
    }

    Ok(quote! {
        #(#additional_types)*

        #[derive(Deserialize)]
        enum #error_en {
            #(#error_variants(#error_models)),*
        }

        #[derive(Deserialize)]
        enum #success_en {
            #(#success_variants(#success_models)),*
        }
    })
}
