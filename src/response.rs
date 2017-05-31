pub use ::reqwest::StatusCode;

pub type ResponseResult = Result<OkResponse, ErrResponse>;
type Body = ::serde::json::Value;
type ReqwestResponse = ::reqwest::Response;

pub trait Response {
    fn from(&mut Option<&mut ReqwestResponse>) -> Self;
    fn body(&self) -> &Body;
    fn status_code(&self) -> StatusCode;
}

impl Response for ResponseResult {
    fn from(maybe_response: &mut Option<&mut ReqwestResponse>) -> Self {
        let error: bool;
        if let Some(ref response) = *maybe_response {
            error = !is_ok(response.status());
        } else {
            error = true;
        }

        match *maybe_response {
            Some(_) => {
                if error {
                    Ok(<OkResponse as Response>::from(maybe_response))
                } else {
                    Err(<ErrResponse as Response>::from(maybe_response))
                }
            },
            None => Err(<ErrResponse as Response>::from(&mut None)),
        }
    }

    fn body(&self) -> &Body {
        match *self {
            Ok(ref s) => <OkResponse as Response>::body(&s),
            Err(ref s) => <ErrResponse as Response>::body(&s),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Ok(ref s) => <OkResponse as Response>::status_code(&s),
            Err(ref s) => <ErrResponse as Response>::status_code(&s),
        }
    }
}

pub fn is_ok(status: &StatusCode) -> bool {
    status.is_success()
        || status.is_informational()
        || status.is_redirection()
}

#[allow(private_in_public)]
struct OkResponse {
    body: Body,
    status_code: StatusCode,
}

impl Response for OkResponse {
    fn from(mut maybe_response: &mut Option<&mut ReqwestResponse>) -> Self {
        let status_code: StatusCode;

        if let Some(ref response) = *maybe_response {
            status_code = *response.status();
            assert!(is_ok(&status_code));
        } else {
            panic!("OkResponse requires Some response.")
        }

        Self {
            body: deser_body(&mut maybe_response),
            status_code,
        }
    }

    fn body(&self) -> &Body {
        &self.body
    }

    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

#[allow(private_in_public)]
struct ErrResponse {
    body: Body,
    status_code: StatusCode,
}

impl Response for ErrResponse {
    fn from(maybe_response: &mut Option<&mut ReqwestResponse>) -> Self {
        let status_code = match *maybe_response {
            Some(ref response) => deser_status(&Some(response)),
            None => deser_status(&None),
        };

        match *maybe_response {
            Some(_) => {
                assert!(!is_ok(&status_code));
                Self {
                    body: deser_body(maybe_response),
                    status_code,
                }
            },
            None => Self {
                body: deser_body(&mut None),
                status_code: deser_status(&None),
            },
        }
    }

    fn body(&self) -> &Body {
        &self.body
    }

    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

fn deser_body(response: &mut Option<&mut ::reqwest::Response>) -> Body {
    match *response {
        Some(ref mut r) => r.json().expect("Malformed JSON response."),
        None => ::serde::json::Value::Null,
    }
}

fn deser_status(response: &Option<&::reqwest::Response>) -> StatusCode {
    match *response {
        Some(r) => *r.status(),
        None => StatusCode::from_u16(520),
    }
}
