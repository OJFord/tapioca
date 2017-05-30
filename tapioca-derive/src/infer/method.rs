use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::parameter;
use infer::TokensResult;

const METHODS: &'static [&'static str] = &[
    "DELETE",
    "GET",
    "HEAD",
    "PATCH",
    "POST",
    "PUT",
];

pub(super) fn valid(method: &str) -> bool {
    METHODS.contains(&method.to_uppercase().as_str())
}

fn trait_ident(method: &str) -> Ident {
    assert!(valid(method), "Invalid method: {}", method);
    Ident::new(format!("Http{}", method.to_class_case()))
}

fn fn_ident(method: &str) -> Ident {
    assert!(valid(method), "Invalid method: {}", method);
    Ident::new(method.to_lowercase())
}

fn query_param_struct_ident(path_st: &Ident, method: &str) -> Ident {
    assert!(valid(method), "Invalid method: {}", method);
    Ident::new(format!("{}{}QueryParams", path_st, method.to_class_case()))
}

pub(super) fn infer_v3(path_st: &Ident, method: &str, schema: &Yaml) -> TokensResult {
    let method_tr = trait_ident(&method);
    let method_fn = fn_ident(&method);
    let query_param_st = query_param_struct_ident(&path_st, &method);

    let query_params: Vec<(Tokens, Tokens)>;
    if let Some(parameters) = schema["parameters"].as_vec() {
        query_params = parameters.iter()
            .filter(|p| p["in"] == Yaml::from_str("query"))
            .map(|p| parameter::infer_v3(p).unwrap())
            .collect();
    } else {
        query_params = Vec::new();
    }

    let (qp_st_fields, qp_types): (Vec<Tokens>, Vec<Tokens>) = (
        query_params.iter().cloned().map(|(p, _)| p).collect(),
        query_params.iter().cloned().map(|(_, t)| t).collect(),
    );
    let qp_strings: Vec<Tokens> = qp_st_fields.iter().cloned()
        .map(|p| quote!{ "#p" }).collect();
    let qp_st_accessors = qp_st_fields.clone();


    Ok(quote! {
        #[allow(dead_code)]
        struct #query_param_st {
            #(#qp_st_fields: #qp_types),*
        }

        #[allow(dead_code)]
        #[allow(unused_variables)]
        impl tapioca::traits::#method_tr for #path_st {
            type QueryParams = #query_param_st;
            type Response = ();

            fn #method_fn(&self, query_parameters: Self::QueryParams) -> Self::Response {
                let mut url = tapioca::Url::parse(*Self::API_URL)
                    .expect("Malformed server URL or path.");
                url.set_path(Self::API_URI);
                url.query_pairs_mut()
                    .clear()
                    #(.append_pair(
                        #qp_strings,
                        query_parameters.#qp_st_accessors.to_string().as_str()
                    ))*
                    ;

                let client = tapioca::Client::new().unwrap();
                client.#method_fn(url).send();
            }
        }
    })
}
