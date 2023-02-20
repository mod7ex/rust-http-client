use crate::error::Error;

#[derive(PartialEq, Debug)]
pub struct UrlParser {
    pub scheme: String,
    pub host: String,
    pub path: String
}

impl UrlParser {
    pub fn from(url: &str) -> Result<UrlParser, Error> {
        let addr = if url.starts_with("http") || url.starts_with("https") {
            url.to_owned()
        } else {
            format!("http://{}", url)
        };

        let mut parts = addr.split("://");

        let scheme = match parts.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::UrlParsingError),
        };

        parts = match parts.next() {
            Some(v) => v.split("/"),
            None => return Err(Error::UrlParsingError),
        };

        let host = match parts.next() {
            Some(v) => v.to_owned(),
            None => return Err(Error::UrlParsingError),
        };

        let mut path = String::new();

        loop {
            match parts.next() {
                Some(v) => {
                    path.push_str(format!("/{}", v).as_str())
                },
                None => {
                    if path.is_empty() {
                        path.push('/');
                    }
                    break;
                }
            }
        }

        Ok(UrlParser {
            host,
            path,
            scheme
        })
    }
}

mod test {
    use super::UrlParser;

    #[test]
    fn test1() {
        let url = "https://example.com";

        let parsed_url = UrlParser::from(url).unwrap();

        let expected = UrlParser {
            scheme: "https".to_owned(),
            host: "example.com".to_owned(),
            path: "/".to_owned()
        };

        assert_eq!(parsed_url, expected);
    }

    #[test]
    fn test2() {
        let url = "example.com/some/path";

        let parsed_url = UrlParser::from(url).unwrap();

        let expected = UrlParser {
            scheme: "http".to_owned(),
            host: "example.com".to_owned(),
            path: "/some/path".to_owned()
        };

        assert_eq!(parsed_url, expected);
    }
}