use std::{collections::HashMap, str::FromStr};

use strum_macros::EnumString;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum Schema {
    #[strum(serialize = "http")]
    Http,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Host(String);

#[derive(Debug, Clone, PartialEq)]
pub struct Port(u16);

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    schema: Schema,
    host: Host,
    port: Port,
    path: String,
    query_strings: HashMap<String, String>,
}

impl Url {
    pub fn new (url: String) -> Self {
        Self {
            schema: Url::extract_schema(&url).unwrap(),
            host: Url::extract_host(&url).unwrap(),
            port: Url::extract_port(&url).unwrap(),
            path: "".to_string(),
            query_strings: HashMap::new(),
        }
    }

    fn extract_schema(url: &String) -> Result<Schema, UrlError> {
        let parsed = Url::parse(url)?;
        let schema = parsed.get("schema")
            .ok_or_else(|| UrlError::InvalidFormatError("schema missing".to_string()))?;
        Schema::from_str(schema).map_err(|e| UrlError::InvalidFormatError(e.to_string()))
    }

    fn extract_host(url: &String) -> Result<Host, UrlError> {
        let parsed = Url::parse(url)?;
        let host = parsed.get("host")
            .ok_or_else(|| UrlError::InvalidFormatError("host missing".to_string()))?
            .to_string();
        Ok(Host(host))
    }

    fn extract_port(url: &String) -> Result<Port, UrlError> {
        let parsed = Url::parse(url)?;
        let port = parsed.get("port")
            .ok_or_else(|| UrlError::InvalidFormatError("port missing".to_string()))?
            .parse::<u16>()
            .map_err(|e| UrlError::InvalidFormatError(e.to_string()))?;
        Ok(Port(port))
    }

    fn parse(url: &String) -> Result<HashMap<String, String>, UrlError> {
        let mut parsed_url = HashMap::new();

        let schema_and_rest = url.split("://").map(|x| x.to_string()).collect::<Vec<String>>();
        match schema_and_rest.first() {
            Some(x) => parsed_url.insert("schema".to_string(), x.to_owned()),
            None => return Err(UrlError::InvalidFormatError(format!("schema missing {}", url).to_string())),
        };

        let url = match schema_and_rest.get(1) {
            Some(x) => x,
            None => return Err(UrlError::InvalidFormatError(format!("missing the part after schema {}", url).to_string())),
        };
        let host_and_rest = url.splitn(2, "/").map(|x| x.to_string()).collect::<Vec<String>>();
        match host_and_rest.first() {
            Some(x) => {
                if let Some(index) = x.find(":") {
                    parsed_url.insert("host".to_string(), x[..index].to_string());
                    parsed_url.insert("port".to_string(), x[index+1..].to_string());
                } else {
                    parsed_url.insert("host".to_string(), x.clone());
                    parsed_url.insert("port".to_string(), 80.to_string());
                };
            },
            None => return Err(UrlError::InvalidFormatError(format!("missing host {}", url).to_string())),
        };

        Ok(parsed_url)
    }
}

#[derive(Error, Debug)]
pub enum UrlError {
    #[error("Invalid url: {0}")]
    InvalidFormatError(String)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_new {
        use super::*;

        #[test]
        fn normal() {
            let actual = Url::new("http://example.com:8080/path/to/resource?q1=A&q2=B".to_string());
            assert_eq!(actual.schema, Schema::Http);
            assert_eq!(actual.host, Host("example.com".to_string()));
            assert_eq!(actual.port, Port(8080));
        }
    }
}
