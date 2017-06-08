use ::inflector::Inflector;
use ::regex::Regex;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::method;
use infer::TokensResult;

fn mod_ident(path: &str) -> Ident {
    let rustified = path.replace('/', " ").trim().to_snake_case();
    let re = Regex::new(r"\{[^/}]+\}").unwrap();
    let ident = re.replace_all(rustified.as_str(), "resource");

    Ident::new(ident)
}

pub(super) fn infer_v3(path: &str, schema: &Yaml) -> TokensResult {
    let path_mod = mod_ident(path);

    let method_schemata = schema.as_hash().expect("Path must be a map.");
    let mut method_impls = quote!{};

    for (method, method_schema) in method_schemata {
        let method = method.as_str().expect("Method must be a string.");
        method_impls.append(method::infer_v3(&method, &method_schema)?);
    }

    Ok(quote! {
        pub(super) mod #path_mod {
            use ::tapioca::Client;
            use ::tapioca::Url;
            use ::tapioca::header;
            use ::tapioca::response::Response;

            use super::schema_ref;
            use super::API_URL;

            const API_PATH: &'static str = #path;

            #method_impls
        }
    })
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn leading_slash() {
        assert_eq!(mod_ident("/foo"), Ident::new("foo"));
    }

    #[test]
    fn both_slash() {
        assert_eq!(mod_ident("/foo/"), Ident::new("foo"));
    }

    #[test]
    fn no_slash() {
        assert_eq!(mod_ident("foo"), Ident::new("foo"));
    }

    #[test]
    fn trailing_slash() {
        assert_eq!(mod_ident("foo/"), Ident::new("foo"));
    }

    #[test]
    fn multipart() {
        assert_eq!(mod_ident("/foo/bar"), Ident::new("foo_bar"));
    }

    #[test]
    fn resource() {
        assert_eq!(mod_ident("/foo/{id}"), Ident::new("foo_resource"));
    }

    #[test]
    fn multi_resource() {
        assert_eq!(mod_ident("/foo/{id}/{bar}"), Ident::new("foo_resource_resource"));
    }

    #[test]
    fn multipart_resource() {
        assert_eq!(mod_ident("/f/{x}/b"), Ident::new("foo_resource_b"));
    }

    #[test]
    fn multipart_multiresource() {
        assert_eq!(mod_ident("/f/{x}/b/{y}"), Ident::new("foo_resource_b_resource"));
    }
}
