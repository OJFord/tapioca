use ::inflector::Inflector;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::TokensResult;

fn struct_ident(api_st: &Ident, path: &str) -> Ident {
    Ident::new(format!("{}{}", api_st, path.replace('/', " ").to_class_case()))
}

fn fn_ident(path: &str) -> Ident {
    Ident::new(path.replace('/', "").to_lowercase())
}

pub(super) fn infer_v3(api_st: &Ident, path: &str, schema: &Yaml) -> TokensResult {
    let path_st = struct_ident(api_st, path);
    let path_fn = fn_ident(path);

    let tokens = quote! {
        struct #path_st;

        impl #api_st {
            fn #path_fn() -> #path_st {
                #path_st
            }
        }

        impl Schema for #path_st {
            fn get(&self) {
                panic!("Not implemented!")
            }
        }
    };

    Ok(tokens)
}
