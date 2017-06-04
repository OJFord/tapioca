use response::ClientResponse;

pub use reqwest::StatusCode;

pub trait Status {
    type OfType;

    fn of(&Option<&Self::OfType>) -> Self;

    fn is_ok(&self) -> bool;
    fn is_err(&self) -> bool;
}

impl<'a> Status for StatusCode {
    type OfType = ClientResponse;

    fn of(response: &Option<&Self::OfType>) -> Self {
        match *response {
            Some(r) => *r.status(),
            None => Self::from_u16(520),
        }
    }

    fn is_ok(&self) -> bool {
        self.is_success()
            || self.is_informational()
            || self.is_redirection()
    }

    fn is_err(&self) -> bool {
        !self.is_ok()
    }
}
