use crate::git::error::GitError;
use std::fmt;

#[derive(Debug)]
pub enum GFBError {
    KeyNotFound(Option<String>),
    KeyAlreadyExists(String),
    AmbiguousKey(),
    GitCommandFailed(String),
    ConfigLoadFailed(String),
    ConfigSavedFailed(String),
    ConfigCreateFailed(String),
    CurrentBranchResolveFailed(String),
    IOError(std::io::Error),
    Git(GitError),
}

impl fmt::Display for GFBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GFBError::KeyNotFound(Some(key)) => write!(f, "No branch saved for key '{}'", key),
            GFBError::KeyNotFound(None) => write!(f, "No key is saved for the current branch"),
            GFBError::KeyAlreadyExists(key) => write!(f, "Key '{}' already exists", key),
            GFBError::AmbiguousKey() => {
                write!(f, "More than one key is saved for the current branch")
            }
            GFBError::GitCommandFailed(msg) => write!(f, "Git command failed: {}", msg),
            GFBError::ConfigLoadFailed(msg) => write!(f, "Failed to load config: {}", msg),
            GFBError::ConfigSavedFailed(msg) => write!(f, "Failed to save config: {}", msg),
            GFBError::ConfigCreateFailed(msg) => write!(f, "Failed to create config: {}", msg),
            GFBError::CurrentBranchResolveFailed(msg) => {
                write!(f, "Failed to get current branch: {}", msg)
            }
            GFBError::IOError(error) => write!(f, "IO error: {}", error),
            GFBError::Git(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for GFBError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GFBError::Git(error) => Some(error),
            GFBError::IOError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<GitError> for GFBError {
    fn from(value: GitError) -> Self {
        GFBError::Git(value)
    }
}

impl From<std::io::Error> for GFBError {
    fn from(value: std::io::Error) -> Self {
        GFBError::IOError(value)
    }
}

pub type Result<T> = std::result::Result<T, GFBError>;
