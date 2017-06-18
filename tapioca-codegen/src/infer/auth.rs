use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::TokensResult;

fn infer_v3_http(scheme_ident: &Ident, schema: &Yaml) -> TokensResult {
    match schema["scheme"].as_str().expect("http security scheme must be a string")
        .to_title_case().as_str()
    {
        "Basic" => Ok(quote! {
            pub type #scheme_ident = header::Basic;

            impl From<(String, String)> for #scheme_ident {
                fn from((username, password): &(String, String)) -> Self {
                    Self { username, password }
                }
            }
        }),
        _ => Err(From::from("currently supported HTTP auth schemes are: Basic")),
    }
}

fn infer_v3_api_key(scheme_ident: &Ident, schema: &Yaml) -> TokensResult {
    let header_name = schema["name"].as_str().expect("apiKey header name must be a string");

    Ok(quote! {
        #[derive(Clone, Debug)]
        pub struct #scheme_ident(String);

        impl From<&'static str> for #scheme_ident {
            fn from(key: &'static str) -> Self {
                Self { 0: key.into() }
            }
        }

        impl From<String> for #scheme_ident {
            fn from(key: String) -> Self {
                Self { 0: key }
            }
        }

        impl header::Header for #scheme_ident {
            fn header_name() -> &'static str {
                #header_name
            }

            fn parse_header(raw: &[Vec<u8>]) -> HeaderResult<#scheme_ident> {
                Ok(Self { 0: String::from_utf8(raw[0].clone())? })
            }
        }

        impl header::HeaderFormat for #scheme_ident {
            fn fmt_header(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(self.0.as_str())
            }
        }
    })
}

pub(super) fn infer_v3_component(scheme_name: &str, schema: &Yaml) -> TokensResult {
    let ident = Ident::from(scheme_name.to_class_case());

    match schema["type"].as_str().expect("security scheme type must be a string")
        .to_camel_case().as_str()
    {
        "http" => infer_v3_http(&ident, &schema),
        "apiKey" => infer_v3_api_key(&ident, &schema),
        _ => Err(From::from("currently supported auth types are: http; apiKey")),
    }
}

pub(super) fn infer_v3(struct_name: &Ident, schema: &Yaml) -> (Tokens, Vec<Ident>) {
    let mut scheme_variants: Vec<Ident> = Vec::new();
    let mut scheme_models: Vec<Tokens> = Vec::new();
    let mut scopes_models: Vec<Tokens> = Vec::new();

    for scheme in schema.as_vec().expect("security requirements must be an array") {
        let scheme = scheme.as_hash().expect("security requirement must be a map");
        let scheme_id = scheme.keys().collect::<Vec<_>>().pop().unwrap();
        let scopes = &scheme[scheme_id];

        let classname = scheme_id.as_str()
            .expect("security scheme identifier must be a string");
        let ident = Ident::from(classname.to_class_case());

        scheme_variants.push(ident.clone());
        scheme_models.push(quote!{ auth_scheme::#ident });

        let mut scopes_model = quote!();
        for scope in scopes.as_vec().expect("scope must be an array") {
            let classname = scope.as_str().expect("scope must be a string")
                .to_class_case();
            let ident = Ident::from(classname);

            scopes_model.append(quote!{ auth_scheme::scope::#ident, });
        }

        scopes_models.push(scopes_model);
    }

    let scheme_variants2 = scheme_variants.clone();
    (
        quote! {
            #[derive(Clone, Debug)]
            pub enum #struct_name {
                #(#scheme_variants(#scheme_models<#scopes_models>),)*
            }
        },
        scheme_variants2
    )
}
