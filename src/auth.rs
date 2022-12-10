use ::base64;
use ::header;
use ::HeaderResult;

#[derive(Clone, Debug)]
pub struct HttpBasic {
    pub user: String,
    pub password: String,
}

impl From<(String, String)> for HttpBasic {
    fn from((user, password): (String, String)) -> Self {
        Self {
            user,
            password,
        }
    }
}

impl ::std::fmt::Display for HttpBasic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let raw = format!("{}:{}", self.user, self.password);
        let encoded = base64::encode(raw.as_str());
        f.write_str(format!("Basic {}", encoded).as_str())
    }
}

impl header::Header for HttpBasic {
    fn header_name() -> &'static str {
        "Authorization"
    }

    fn parse_header(raw: &header::Raw) -> HeaderResult<HttpBasic> {
        let encoded = &raw[0];
        let decoded = String::from_utf8(base64::decode(encoded).unwrap())?;
        let parts = decoded.split(':').collect::<Vec<_>>();

        Ok(Self { user: parts[0].into(), password: parts[1].into() })
    }

    fn fmt_header(&self, f: &mut header::Formatter) -> ::std::fmt::Result {
        f.fmt_line(self)
    }
}
