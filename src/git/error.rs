use std::{error::Error, fmt, io, string::FromUtf8Error};

#[derive(Debug)]
pub enum GitError {
    Io(io::Error),
    Libgit(git2::Error),
    Utf8(FromUtf8Error),
    Failed(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::Io(err) => write!(f, "could not run git: {}", err),
            GitError::Libgit(err) => write!(f, "{}", err.message()),
            GitError::Utf8(_) => write!(f, "git output was not valid UTF-8"),
            GitError::Failed(message) => write!(f, "{}", message),
        }
    }
}

impl Error for GitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            GitError::Io(err) => Some(err),
            GitError::Libgit(err) => Some(err),
            GitError::Utf8(err) => Some(err),
            GitError::Failed(_) => None,
        }
    }
}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> Self {
        GitError::Io(err)
    }
}

impl From<git2::Error> for GitError {
    fn from(err: git2::Error) -> Self {
        GitError::Libgit(err)
    }
}

impl From<FromUtf8Error> for GitError {
    fn from(err: FromUtf8Error) -> Self {
        GitError::Utf8(err)
    }
}
