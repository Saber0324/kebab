pub use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
pub use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const PISTON_URL: &str = "http://localhost:2000";

#[derive(Serialize, Debug)]
pub struct ExecuteRequest {
    language: String,
    version: String,
    files: Vec<File>,
    stdin: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct File {
    content: String,
}

#[derive(Deserialize)]
#[expect(dead_code)]
pub struct ExecutionResult {
    language: String,
    version: String,
    run: RunResult,
}

#[derive(Deserialize, Debug)]
pub struct RunResult {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub code: u8,
}

#[derive(Error, Debug)]
#[expect(dead_code)]
pub enum ExecutionError {
    #[error("Failed to get or send the request to Piston:\n")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Execution timed out waiting for results\n")]
    Timeout,
}

pub fn build_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

pub async fn submit_code(
    client: &reqwest::Client,
    language: String,
    code: String,
    stdin: Option<String>,
) -> Result<RunResult, ExecutionError> {
    let payload = ExecuteRequest {
        language,
        version: "*".to_string(),
        files: vec![File { content: code }],
        stdin,
    };

    let url = format!("{PISTON_URL}/api/v2/execute");
    let res = client
        .post(&url)
        .headers(build_headers())
        .json(&payload)
        .send()
        .await?
        .json::<ExecutionResult>()
        .await?;

    Ok(res.run)
}
