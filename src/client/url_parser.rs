use crate::error::Error;

#[derive(PartialEq, Debug)]
pub struct UrlParser {
    pub scheme: String,
    pub hostname: String,
    pub port: usize,
    pub path: String
}

impl UrlParser {
    pub fn from(url: &str) -> Result<UrlParser, Error> {
        let addr = if url.starts_with("http") || url.starts_with("https") {
            url.to_owned()
        } else {
            format!("http://{}", url)
        };

        let mut port = 80;

        let mut parts = addr.split("://");

        let scheme = match parts.next() {
            Some(v) => {
                match v {
                    "https" => { port = 443; },
                    _ => {}
                }

                v.to_string()
            },
            None => return Err(Error::UrlParsingError("Scheme".to_owned())),
        };

        parts = match parts.next() {
            Some(v) => v.split("/"),
            None => return Err(Error::UrlParsingError("host and path".to_owned())),
        };

        let hostname = match parts.next() {
            Some(v) => {
                let mut host_name = v.to_string();

                if v.contains(":") {
                    let mut payload = v.split(":");
                    
                    host_name = payload.next().unwrap().to_string();
                    port = payload.next().unwrap().parse().unwrap_or(80);
                }
                
                host_name
            },
            None => return Err(Error::UrlParsingError("hostname".to_owned())),
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
            hostname,
            path,
            port,
            scheme
        })
    }
}

#[cfg(test)]
mod test {
    use super::UrlParser;

    #[test]
    fn test1() {
        let url = "https://example.com";

        let parsed_url = UrlParser::from(url).unwrap();

        let expected = UrlParser {
            scheme: "https".to_owned(),
            hostname: "example.com".to_owned(),
            port: 443,
            path: "/".to_owned()
        };

        assert_eq!(parsed_url, expected);
    }

    #[test]
    fn test2() {
        let url = "http://example.com:3000/some/path";

        let parsed_url = UrlParser::from(url).unwrap();

        println!("--------------> {:?}", parsed_url);

        let expected = UrlParser {
            scheme: "http".to_owned(),
            hostname: "example.com".to_owned(),
            port: 3000,
            path: "/some/path".to_owned()
        };

        assert_eq!(parsed_url, expected);
    }
}