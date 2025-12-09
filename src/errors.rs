use std::fmt;

#[derive(Debug)]
pub enum GFBError {
    KeyNotFound(String),
    GitCommandFailed(String),
    ConfigLoadFailed(String),
    ConfigSavedFailed(String),
    ConfigCreateFailed(String),
    CurrentBranchResolveFailed(String),
    IOError(std::io::Error),
}

impl fmt::Display for GFBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GFBError::KeyNotFound(key) => write!(f, "No branch saved for key '{}'", key),
            GFBError::GitCommandFailed(msg) => write!(f, "Git command failed: {}", msg),
            GFBError::ConfigLoadFailed(msg) => write!(f, "Failed to load config: {}", msg),
            GFBError::ConfigSavedFailed(msg) => write!(f, "Failed to save config: {}", msg),
            GFBError::ConfigCreateFailed(msg) => write!(f, "Failed to create config: {}", msg),
            GFBError::CurrentBranchResolveFailed(msg) => write!(f, "Failed to get current branch: {}", msg),
            GFBError::IOError(error) => write!(f, "IO error: {}", error),
        }
    }
}

impl std::error::Error for GFBError {}

impl From<std::io::Error> for GFBError {
    fn from(value: std::io::Error) -> Self {
        GFBError::IOError(value)
    }
}

pub type Result<T> = std::result::Result<T, GFBError>;
