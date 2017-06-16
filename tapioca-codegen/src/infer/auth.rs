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
                pub fn from((username, password): &(String, String)) -> Self {
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
        #[derive(Clone, Copy, Debug)]
        pub struct #scheme_ident(String);

        impl From<String> for #scheme_ident {
            pub fn from(key: String) -> Self {
                Self { 0: key }
            }
        }

        impl header::Header for #scheme_ident {
            fn header_name() -> &'static str {
                #header_name
            }

            fn parse_header(raw: &[Vec<u8>]) -> HeaderResult<#scheme_ident> {
                if raw.len() == 1 {
                    Ok(Self { 0: String::from_utf8(raw[0])? })
                } else {
                    Err(From::from(format!("Multiple auth headers {}", #scheme_ident)))
                }
            }
        }

        impl header::HeaderFormat for #scheme_ident {
            fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(self.0)
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

pub(super) fn infer_v3(struct_name: &Ident, schema: &Yaml) -> TokensResult {
    let mut scheme_variants: Vec<Ident> = Vec::new();
    let mut scheme_models: Vec<Tokens> = Vec::new();
    let mut scopes_models: Vec<Tokens> = Vec::new();

    for (scheme, scopes) in schema.as_hash().expect("security requirements must be a map") {
        let classname = scheme.as_str().expect("security scheme must be a string")
            .to_class_case();
        let ident = Ident::from(classname);

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

    Ok(quote! {
        pub enum #struct_name {
            #(#scheme_variants(#scheme_models<#scopes_models>),)*
        }
    })
}
