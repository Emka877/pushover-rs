use std::{io::BufReader, path::PathBuf};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TestData {
    #[serde(alias = "token")]
    pub app_token: String,

    #[serde(alias = "user")]
    pub user_key: String,
}

pub fn read_test_data() -> Result<TestData, Box<dyn std::error::Error>> {
    let file_path: PathBuf = PathBuf::from("testdata/credentials.json");
    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}
