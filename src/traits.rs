pub trait HttpDelete {
    type QueryParams;
    type Response;

    fn delete(&self, qp: Self::QueryParams) -> Self::Response;
}

pub trait HttpGet {
    type QueryParams;
    type Response;

    fn get(&self, qp: Self::QueryParams) -> Self::Response;
}

pub trait HttpHead {
    type QueryParams;
    type Response;

    fn head(&self, qp: Self::QueryParams) -> Self::Response;
}

pub trait HttpPatch {
    type QueryParams;
    type Response;

    fn patch(&self, qp: Self::QueryParams) -> Self::Response;
}

pub trait HttpPost {
    type QueryParams;
    type Response;

    fn post(&self, qp: Self::QueryParams) -> Self::Response;
}

pub trait HttpPut {
    type QueryParams;
    type Response;

    fn put(&self, qp: Self::QueryParams) -> Self::Response;
}
