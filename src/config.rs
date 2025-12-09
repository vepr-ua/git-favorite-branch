use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use crate::errors::{GFBError, Result};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub state: HashMap<String, String>,
    #[serde(default)]
    pub path_to_config: String,
}

impl Config {
    pub fn new(path: &str, state: HashMap<String, String>) -> Self {
        Config {
            state,
            path_to_config: path.to_string(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let file = File::create(&self.path_to_config)
            .map_err(|e| GFBError::ConfigSavedFailed(e.to_string()))?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)
            .map_err(|e| GFBError::ConfigSavedFailed(e.to_string()))?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let file_path = Path::new(&self.path_to_config);
        let file = self.create_file_if_none_exist(file_path)?;

        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)
            .map_err(|e| GFBError::ConfigLoadFailed(e.to_string()))?;

        self.state = config.state;
        self.path_to_config = config.path_to_config;
        Ok(())
    }

    fn create_file_if_none_exist(&self, formatted_file_path: &Path) -> Result<File> {
        if let Ok(file) = File::open(formatted_file_path) {
            return Ok(file);
        }

        let new_file = File::create(formatted_file_path)
            .map_err(|e| GFBError::ConfigCreateFailed(e.to_string()))?;

        let writer = BufWriter::new(&new_file);
        serde_json::to_writer_pretty(writer, &self)
            .map_err(|e| GFBError::ConfigCreateFailed(e.to_string()))?;

        File::open(formatted_file_path).map_err(|_| {
            GFBError::ConfigCreateFailed("Unable to open newly created file".to_string())
        })
    }
}
