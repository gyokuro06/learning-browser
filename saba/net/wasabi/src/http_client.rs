use alloc::string::String;
use noli::net::lookup_host;

use crate::{http_error::HttpError, http_response::HttpResponse};

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, HttpError> {
        let ip_addresses = match lookup_host(&host) {
            Ok(x) => x,
            Error(e) => {
                return Err(HttpError::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                    e
                )))
            }
        }

        if ip_addresses.len() < 1 {
            return Err(HttpError::Network("No IP Addresses were found"))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     mod get {
//         use super::*;

//         #[test]
//         fn test_normal() {
//             let actual =
//         }
//     }
// }
