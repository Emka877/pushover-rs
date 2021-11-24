use std::fs::File;
use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExampleCredentials {
    pub user: String,
    pub token: String,
}

pub fn read_credentials() -> ExampleCredentials {
    let path: String = "examples/data/credentials.ron".into();
    let file: File = File::open(&path)
        .expect(&format!("Cannot find file at location: {}", path));
    match from_reader(file) {
        Ok(creds) => creds,
        Err(err) => {
            eprintln!("Error reading credentials file: {}", err);
            std::process::exit(1);
        },
    }
}
