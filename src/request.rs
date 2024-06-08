use core::fmt;

use reqwest;
use crate::io_read::CRequest;

// #[derive(Debug)]
pub struct MethodError;
impl fmt::Display for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid method entered!")
    }
}

pub async fn cwes_request(crequest: CRequest) -> Result<reqwest::StatusCode, MethodError> {

    let client = reqwest::Client::new();

    match crequest.get_method() {
        "get" => {
            let response = client
                .get(crequest.get_full())
                .send()
                .await
                .expect("Could not execute GET request!");
            return Ok(response.status());
        }
        "post" => {
            let response = client
                .post(crequest.get_full())
                .send()
                .await
                .expect("Could not execute send request!");
            return Ok(response.status());
        }

        _ => Err(MethodError)
        }
}


// Result<Reqwest::StatusCode, MethodError>  Ok(response.status())