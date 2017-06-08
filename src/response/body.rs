use response::Status;
use response::StatusCode;
use response::ClientResponse;

pub trait ResponseBody {
    fn from(&mut Option<&mut ClientResponse>) -> Self;
}

// O: ResponseBody, E: ResponseBody c.f. rust-lang/rust#21903
pub type ResponseResultBody<O, E> = Result<O, E>;

impl<O, E> ResponseBody for ResponseResultBody<O, E>
    where O: ResponseBody, E: ResponseBody,
{
    fn from(maybe_response: &mut Option<&mut ClientResponse>) -> Self {
        let error = match *maybe_response {
            Some(ref response) => StatusCode::of(&Some(response)).is_err(),
            None => true,
        };

        if error {
            Err(<E as ResponseBody>::from(maybe_response))
        } else {
            Ok(<O as ResponseBody>::from(maybe_response))
        }
    }
}
