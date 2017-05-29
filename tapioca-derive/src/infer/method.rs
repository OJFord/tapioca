use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::TokensResult;

fn fn_ident(method: &str) -> Ident {
    Ident::new(method)
}

pub(super) fn infer_v3(path_st: &Ident, method: &str, schema: &Yaml) -> TokensResult {
    let method_fn = fn_ident(method);

    let tokens = quote! {
        impl Schema for #path_st {
            fn #method_fn(&self) {
                panic!("Not implemented!")
            }
        }
    };

    Ok(tokens)
}
