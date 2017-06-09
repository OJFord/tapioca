use query::QueryPair;
use query::QueryParameter;

pub type Required<T> = T;
pub type Optional<T> = Option<T>;

pub(crate) trait TapiocaDatatype: QueryParameter {}


impl TapiocaDatatype for bool {}
impl QueryParameter for bool {
    fn as_query_kv(&self, key: &str) -> Vec<QueryPair> {
        if *self {
            vec![(key.into(), "".into())]
        } else {
            Vec::new()
        }
    }
}

impl TapiocaDatatype for f32 {}
impl QueryParameter for f32 {
    fn as_query_kv(&self, key: &str) -> Vec<QueryPair> {
        vec![(key.into(), self.to_string())]
    }
}

impl TapiocaDatatype for f64 {}
impl QueryParameter for f64 {
    fn as_query_kv(&self, key: &str) -> Vec<QueryPair> {
        vec![(key.into(), self.to_string())]
    }
}

impl TapiocaDatatype for i32 {}
impl QueryParameter for i32 {
    fn as_query_kv(&self, key: &str) -> Vec<QueryPair> {
        vec![(key.into(), self.to_string())]
    }
}

impl TapiocaDatatype for i64 {}
impl QueryParameter for i64 {
    fn as_query_kv(&self, key: &str) -> Vec<QueryPair> {
        vec![(key.into(), self.to_string())]
    }
}

impl TapiocaDatatype for String {}
impl QueryParameter for String {
    fn as_query_kv(&self, key: &str) -> Vec<QueryPair> {
        vec![(key.into(), self.clone())]
    }
}
