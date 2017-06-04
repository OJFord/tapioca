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

pub(super) fn infer_v3(schema: &Yaml) -> TokensResult {
    let mut err_codes: Vec<u16> = Vec::new();
    let mut err_variants: Vec<Ident> = Vec::new();
    let mut err_models: Vec<Tokens> = Vec::new();

    let mut ok_codes: Vec<u16> = Vec::new();
    let mut ok_variants: Vec<Ident> = Vec::new();
    let mut ok_models: Vec<Tokens> = Vec::new();

    let mut additional_types: Vec<Tokens> = Vec::new();

    for (code, schema) in schema.as_hash().expect("Responses must be a map.") {
        let (status_code, status_str) = parse_response_key(&code);
        let variant_ident = Ident::new(status_str);

        let inferred_type: Tokens;
        let additional_type: Option<Tokens>;

        let schema = &schema["content"]["application/json"]["schema"];
        if let None = schema.as_hash() {
            inferred_type = quote!{ () };
            additional_type = None;
        } else {
            let (ty, at) = datatype::infer_v3(&schema)?;
            inferred_type = ty;
            additional_type = at;
        }

        if let Some(t) = additional_type {
            additional_types.push(t);
        }

        if status_code < 400 {
            ok_codes.push(status_code);
            ok_variants.push(variant_ident);
            ok_models.push(inferred_type);
        } else {
            err_codes.push(status_code);
            err_variants.push(variant_ident);
            err_models.push(inferred_type);
        }
    }

    let ok_models2 = ok_models.clone();
    let err_models2 = err_models.clone();
    let ok_variants2 = ok_variants.clone();
    let err_variants2 = err_variants.clone();
    Ok(quote! {
        use ::tapioca::response::ClientResponse;
        use ::tapioca::response::Response;
        use ::tapioca::response::ResponseBody;
        use ::tapioca::response::ResponseResult as _ResponseResult;
        use ::tapioca::response::Status;
        use ::tapioca::response::StatusCode;
        use ::tapioca::serde::de::DeserializeOwned;
        use ::tapioca::serde_json;

        #(
            #[derive(Clone, Deserialize)]
            #additional_types
        )*

        pub type ResponseResult = _ResponseResult<OkResult, ErrResult>;


        #[derive(Clone)]
        pub enum OkBody {
            #(#ok_variants(#ok_models)),*
        }

        #[derive(Clone)]
        pub enum ErrBody {
            #(#err_variants(#err_models),)*
            NetworkFailure(),
        }

        pub struct OkResult {
            body: OkBody,
            status_code: StatusCode,
        }

        pub struct ErrResult {
            body: ErrBody,
            status_code: StatusCode,
        }

        impl Response for OkResult {
            type BodyType = OkBody;

            fn from(maybe_response: &mut Option<&mut ClientResponse>) -> Self {
                let status_code = match *maybe_response {
                    Some(ref response) => StatusCode::of(&Some(response)),
                    None => panic!("OkResponse requires Some response."),
                };

                assert!(status_code.is_ok());

                Self {
                    body: <OkBody as ResponseBody>::from(maybe_response),
                    status_code,
                }
            }

            fn body(&self) -> Self::BodyType {
                self.body.clone()
            }

            fn status_code(&self) -> StatusCode {
                self.status_code
            }
        }

        impl Response for ErrResult {
            type BodyType = ErrBody;

            fn from(maybe_response: &mut Option<&mut ClientResponse>) -> Self {
                let status_code = match *maybe_response {
                    Some(ref response) => StatusCode::of(&Some(response)),
                    None => StatusCode::of(&None),
                };
                assert!(status_code.is_err());

                Self {
                    body: <ErrBody as ResponseBody>::from(maybe_response),
                    status_code,
                }
            }

            fn body(&self) -> Self::BodyType {
                self.body.clone()
            }

            fn status_code(&self) -> StatusCode {
                self.status_code
            }
        }


        impl ResponseBody for OkBody {
            fn from(maybe_response: &mut Option<&mut ClientResponse>) -> Self {
                let status_code = match *maybe_response {
                    Some(ref response) => StatusCode::of(&Some(response)),
                    None => panic!("OkResponse requires Some response"),
                }.to_u16();

                match (maybe_response, status_code) {
                    #((
                        &mut Some(ref mut response), #ok_codes
                    ) => OkBody::#ok_variants2(
                        response.json::<#ok_models2>().expect("Malformed JSON")
                    ),)*
                    (_, _) => panic!("Unexpected code {}", status_code),
                }
            }
        }

        impl ResponseBody for ErrBody {
            fn from(maybe_response: &mut Option<&mut ClientResponse>) -> Self {
                let status_code = match *maybe_response {
                    Some(ref response) => StatusCode::of(&Some(response)),
                    None => StatusCode::of(&None),
                }.to_u16();

                match (maybe_response, status_code) {
                    #((
                        &mut Some(ref mut response), #err_codes
                    ) => ErrBody::#err_variants2(
                        response.json::<#err_models2>().expect("Malformed JSON")
                    ),)*
                    (&mut Some(_), _) => panic!("Unexpected code {}", status_code),
                    (&mut None, _) => ErrBody::NetworkFailure(),
                }
            }
        }
    })
}
