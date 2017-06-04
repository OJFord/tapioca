use response::Status;
use response::StatusCode;
use response::ClientResponse;

pub trait ResponseBody {
    fn from(&mut Option<&mut ClientResponse>) -> Self;
}

pub type ResponseResultBody<O: ResponseBody, E: ResponseBody> = Result<O, E>;

impl<O, E> ResponseBody for ResponseResultBody<O, E>
    where O: ResponseBody, E: ResponseBody,
{
    fn from(maybe_response: &mut Option<&mut ClientResponse>) -> Self {
        let error = match *maybe_response {
            Some(ref response) => StatusCode::of(&Some(response)).is_err(),
            None => true,
        };

        match error {
            false => Ok(<O as ResponseBody>::from(maybe_response)),
            true => Err(<E as ResponseBody>::from(maybe_response)),
        }
    }
}
