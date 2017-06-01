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
    let borrow = quote!(#[serde(borrow)]);

    let mut error_variants: Vec<Ident> = Vec::new();
    let mut error_models: Vec<Tokens> = Vec::new();
    let mut error_lifetime: Option<Ident> = None;

    let mut success_variants: Vec<Ident> = Vec::new();
    let mut success_models: Vec<Tokens> = Vec::new();
    let mut success_lifetime: Option<Ident> = None;

    let mut additional_types: Vec<Tokens> = Vec::new();

    for (code, schema) in schema.as_hash().expect("Responses must be a map.") {
        let (status_code, status_str) = parse_response_key(&code);
        let variant_ident = Ident::new(status_str);

        let inferred_type: Tokens;
        let additional_type: Option<Tokens>;
        let lifetime: Option<Ident>;

        let schema = &schema["content"]["application/json"]["schema"];
        if let None = schema.as_hash() {
            inferred_type = quote!{ () };
            lifetime = None;
            additional_type = None;
        } else {
            let (ty, lt, at) = datatype::infer_v3(&schema)?;
            inferred_type = ty;
            lifetime = lt;
            additional_type = at;
        }

        if let Some(t) = additional_type {
            additional_types.push(t);
        }

        if status_code < 400 {
            success_variants.push(variant_ident);
            success_lifetime = success_lifetime.or(lifetime.clone());

            if lifetime.is_some() {
                success_models.push(quote!{ #borrow #inferred_type });
            } else {
                success_models.push(inferred_type);
            }
        } else {
            error_variants.push(variant_ident);
            error_lifetime = error_lifetime.or(lifetime.clone());

            if lifetime.is_some() {
                error_models.push(quote!{ #borrow #inferred_type });
            } else {
                error_models.push(inferred_type);
            }
        }
    }

    Ok(quote! {
        #(
            #[derive(Deserialize)]
            #additional_types
        )*

        #[derive(Deserialize)]
        enum #error_en<#error_lifetime> {
            #(#error_variants(#error_models)),*
        }

        #[derive(Deserialize)]
        enum #success_en<#success_lifetime> {
            #(#success_variants(#success_models)),*
        }
    })
}
