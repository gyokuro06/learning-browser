use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    pub url: String,
    pub host: String,
    pub port: String,
    pub path: String,
    pub search_part: String,
}

// FIXME
impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            search_part: "".to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        let host = Url::extract_host(&self.url)?;
        self.host = host;

        let port = Url::extract_port(&self.url)?;
        self.port = port;

        let path = Url::extract_path(&self.url)?;
        self.path = path;

        let search_part = Url::extract_search_part(&self.url)?;
        self.search_part = search_part;

        Ok(self.clone())
    }

    fn extract_host(url: &String) -> Result<String, String> {
        let url_parts = url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect::<Vec<&str>>();

        if let Some(index) = url_parts[0].find(":") {
            Ok(url_parts[0][..index].to_string())
        } else {
            Ok(url_parts[0].to_string())
        }
    }

    fn extract_port(url: &String) -> Result<String, String> {
        let url_parts = url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect::<Vec<&str>>();

        if let Some(index) = url_parts[0].find(":") {
            Ok(url_parts[0][index + 1..].to_string())
        } else {
            Ok("80".to_string())
        }
    }

    fn extract_path(url: &String) -> Result<String, String> {
        let url_parts = url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect::<Vec<&str>>();

        if url_parts.len() == 2 {
            Ok("/".to_string() + url_parts[1])
        } else {
            Ok("/".to_string())
        }
    }

    fn extract_search_part(url: &String) -> Result<String, String> {
        let url_parts = url
            .trim_start_matches("http://")
            .splitn(2, "?")
            .collect::<Vec<&str>>();

        if url_parts.len() == 2 {
            Ok("?".to_string() + url_parts[1])
        } else {
            Ok("".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_host {
        use super::*;

        #[test]
        fn test_parse_host() {
            let actual = Url::new("http://example.com".to_string()).parse().unwrap();
            assert_eq!(actual.host, "example.com".to_string());
        }
    }

    mod parse_port {
        use super::*;

        #[test]
        fn test_normal() {
            let actual = Url::new("http://example.com:8080".to_string()).parse().unwrap();
            assert_eq!(actual.port, "8080".to_string());
        }

        #[test]
        fn test_default_port() {
            let actual = Url::new("http://example.com".to_string()).parse().unwrap();
            assert_eq!(actual.port, "80".to_string());
        }
    }

    mod parse_path {
        use super::*;

        #[test]
        fn test_normal() {
            let actual = Url::new("http://example.com:8080/path/to/resource".to_string()).parse().unwrap();
            assert_eq!(actual.path, "/path/to/resource".to_string());
        }
    }

    mod parse_search_part {
        use super::*;

        #[test]
        fn test_normal() {
            let actual = Url::new("http://example.com:8080/path/to/resource?A=1&B=2".to_string()).parse().unwrap();
            assert_eq!(actual.search_part, "?A=1&B=2".to_string());
        }
    }
}
