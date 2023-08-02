#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub connection_string: String,
}
