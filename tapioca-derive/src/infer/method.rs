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

fn fn_ident(method: &str) -> Ident {
    assert!(valid(method), "Invalid method: {}", method);
    Ident::new(method.to_lowercase())
}

fn query_param_struct_ident(method: &str) -> Ident {
    assert!(valid(method), "Invalid method: {}", method);
    Ident::new(format!("{}QueryParams", method.to_class_case()))
}

pub(super) fn infer_v3(method: &str, schema: &Yaml) -> TokensResult {
    let method_fn = fn_ident(&method);
    let query_param_st = query_param_struct_ident(&method);

    let query_params: Vec<(Tokens, Tokens)>;
    let signature: Tokens;
    if let Some(parameters) = schema["parameters"].as_vec() {
        query_params = parameters.iter()
            .filter(|p| p["in"] == Yaml::from_str("query"))
            .map(|p| parameter::infer_v3(p).unwrap())
            .collect();
        signature = quote!{ query_parameters: &#query_param_st };
    } else {
        query_params = Vec::new();
        signature = quote!{};
    }

    let (query_param_fields, query_param_types): (Vec<Tokens>, Vec<Tokens>) = (
        query_params.iter().cloned().map(|(p, _)| p).collect(),
        query_params.iter().cloned().map(|(_, t)| t).collect(),
    );
    let query_param_strings: Vec<Tokens> = query_param_fields.iter().cloned()
        .map(|p| quote!{ "#p" }).collect();
    let query_param_accessors: Vec<Tokens> = query_param_fields.iter().cloned()
        .map(|f| quote!{ query_parameters.#f }).collect();

    Ok(quote! {
        type Response = ();

        #[allow(dead_code)]
        pub(in super::super) struct #query_param_st {
            #(pub(in super::super) #query_param_fields: #query_param_types),*
        }

        #[allow(dead_code)]
        pub(in super::super) fn #method_fn(#signature) -> Response {
            let mut url = tapioca::Url::parse(self::API_URL)
                .expect("Malformed server URL or path.");
            url.set_path(self::API_PATH);
            url.query_pairs_mut()
                .clear()
                #(.append_pair(
                    #query_param_strings,
                    #query_param_accessors.to_string().as_str()
                ))*
                ;

            let client = tapioca::Client::new().unwrap();
            client.#method_fn(url).send();
        }
    })
}
