/// Data structures for parsing consumers data.
pub mod data {
    use serde::{Deserialize, Serialize};

    /// Consumers topic entry.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Info {
        #[serde(rename = "id")]
        pub id: String,

        #[serde(rename = "title")]
        pub title: String,

        #[serde(rename = "article")]
        pub article: String,
    }
}

/// Reader to loading consumers data.
pub mod reader {
    use super::data::Info;
    use crate::errors::IoOrSerdeError;

    /// Loads the consumers data from a file.
    ///
    /// # Errors
    ///
    /// Returns `Err` if fails to read from `path` or parse the contents.
    pub fn parse<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Info>, IoOrSerdeError> {
        let contents = std::fs::read_to_string(path)?;
        let parsed: Vec<Info> = serde_yaml::from_str(&contents)?;
        Ok(parsed)
    }
}
