pub use self::body::ResponseBody;
pub use self::body::ResponseResultBody;
pub use self::status::Status;
pub use self::status::StatusCode;

pub mod body;
pub mod status;

pub type ClientResponse = ::reqwest::Response;
// O: Response, E: Response c.f. rust-lang/rust#21903
pub type ResponseResult<O, E> = Result<O, E>;

pub trait Response {
    type BodyType: ResponseBody;

    fn from(&mut Option<&mut ClientResponse>) -> Self;

    fn body(&self) -> Self::BodyType;
    fn status_code(&self) -> StatusCode;

    fn is_ok(&self) -> bool {
        self.status_code().is_ok()
    }
}

impl<OB: ResponseBody, EB: ResponseBody, O, E> Response for ResponseResult<O, E>
    where O: Response<BodyType=OB>, E: Response<BodyType=EB>
{
    type BodyType = ResponseResultBody<OB, EB>;

    fn from(maybe_response: &mut Option<&mut ClientResponse>) -> Self {
        let error = match *maybe_response {
            Some(ref response) => StatusCode::of(&Some(response)).is_err(),
            None => true,
        };

        match *maybe_response {
            Some(_) => if error {
                Err(E::from(maybe_response))
            } else {
                Ok(O::from(maybe_response))
            },
            None => Err(E::from(&mut None)),
        }
    }

    fn body(&self) -> Self::BodyType {
        match *self {
            Ok(ref s) => Ok(O::body(s)),
            Err(ref s) => Err(E::body(s)),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Ok(ref s) => O::status_code(s),
            Err(ref s) => E::status_code(s),
        }
    }
}
