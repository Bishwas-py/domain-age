use serde::Deserialize;
use reqwest::Error as RequestError;
use serde_json::Error as JsonError;

#[derive(Deserialize)]
pub struct DomainResult {
    pub creation: DomainCreation,
    pub input: String,
    pub tld: String,
    #[serde(rename = "mysticowner")]
    pub mystic_owner: bool,
    #[serde(rename = "iptyBlacklisted")]
    pub ipty_blacklisted: String,
    #[serde(rename = "whoisdata")]
    pub whois_data: String,
}

#[derive(Deserialize)]
pub struct DomainCreation {
    pub years: i32,
    pub days: i32,
    pub fulldays: i32,
    pub classification: String,
    pub field: String,
    pub date: String,
}

#[derive(Deserialize)]
pub struct PreparedDomainResult {
    pub result: DomainResult,
}

#[derive(Debug)]
pub enum DomainParsingError {
    RequestError(RequestError),
    JsonError(JsonError),
}

impl From<RequestError> for DomainParsingError {
    fn from(err: RequestError) -> DomainParsingError {
        DomainParsingError::RequestError(err)
    }
}

impl From<JsonError> for DomainParsingError {
    fn from(err: JsonError) -> DomainParsingError {
        DomainParsingError::JsonError(err)
    }
}

pub async fn get_domain_age(site_domain: &str) -> Result<PreparedDomainResult, DomainParsingError> {
    let api_url = format!("https://ipty.de/domage/api.php?domain={}&mode=full", site_domain);
    let response = reqwest::Client::new().get(api_url).send().await?;
    let response_text = &response.text().await?;
    let prepared_result = serde_json::from_str::<PreparedDomainResult>(response_text)?;
    Ok(prepared_result)
}