use serde::Deserialize;

/**
 Data of the response given by the Pushover API. (if any) 
 **/
#[derive(Debug, Clone, Deserialize)]
pub struct PushoverResponse {
    /* Always present in response */
    /// 1 if the request was valid, 0 otherwise. (Not the HTTP status)
    pub status: u8,
    /// The request GUID
    pub request: String,

    /* Not always present */
    /// Specifies a "user key" error type
    pub user: Option<String>,
    /// Specifies a "app token" error type
    pub token: Option<String>,
    /// A list of error messages
    pub errors: Option<Vec<String>>,
}

impl PushoverResponse {
    pub async fn try_from_reqwest_response(response: reqwest::Response) -> Result<PushoverResponse, Box<dyn std::error::Error>> {
        let data = response.json::<PushoverResponse>().await;

        if data.is_err() {
            return Err(Box::from(data.err().unwrap()))
        }

        return Ok(data.ok().unwrap());
    }

    pub fn try_from_blocking_reqwest_response(response: reqwest::blocking::Response) -> Result<PushoverResponse, Box<dyn std::error::Error>> {
        let data = response.json::<PushoverResponse>();

        if data.is_err() {
            return Err(Box::from(data.err().unwrap()))
        }

        return Ok(data.ok().unwrap());
    }
}
