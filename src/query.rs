pub type QueryPair = (String, String);

pub trait QueryString {
    fn as_query_kv(&self) -> Vec<QueryPair>;

    fn as_query(&self) -> String {
        let params: Vec<String> = self.as_query_kv().iter()
            .map(|&(ref k, ref v)| format!("{}={}", k, v))
            .collect();

        format!("?{}", params.join("&"))
    }
}

pub trait QueryParameter {
    fn as_query_kv(&self, &str) -> Vec<QueryPair>;
}

impl<T: QueryString> QueryString for Option<T> {
    fn as_query_kv(&self) -> Vec<QueryPair> {
        match *self {
            Some(ref thing) => thing.as_query_kv(),
            None => Vec::new(),
        }
    }
}

impl<T: QueryParameter> QueryParameter for Option<T> {
    fn as_query_kv(&self, key: &str) -> Vec<QueryPair> {
        match *self {
            Some(ref thing) => thing.as_query_kv(key),
            None => vec![(key.into(), "".into())],
        }
    }
}

impl<T: QueryParameter> QueryParameter for Vec<T> {
    fn as_query_kv(&self, key: &str) -> Vec<QueryPair> {
        let mut query_pairs: Vec<QueryPair> = Vec::new();
        for item in self.iter() {
            query_pairs.append(&mut item.as_query_kv(key));
        }
        query_pairs
    }
}
