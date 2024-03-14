use serde::Deserialize;
use std::fs::File;
use std::io::Read;
#[derive(Deserialize)]
pub struct Config {
    pub version: f32,
    pub middleware: String,
    pub data_processing_server_addr: String,
    pub data_processing_server_port: i32,
    pub visualization_server_addr: String,
    pub visualization_server_port: i32,
}

pub struct Parser {
    pub config: Config,
}

impl Parser {
    pub fn new(config_file_path: &str) -> Result<Parser, Box<dyn std::error::Error>> {
        let mut config_file = File::open(config_file_path).unwrap();
        let mut config_file_content = String::new();
        config_file
            .read_to_string(&mut config_file_content)
            .unwrap();

        let config: Config = serde_json::from_str(&config_file_content).unwrap();

        Ok(Parser { config: config })
    }
}
