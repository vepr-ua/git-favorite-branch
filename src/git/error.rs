use std::{error::Error, fmt, io, string::FromUtf8Error};

use crate::git::result::GitResult;

#[derive(Debug)]
pub enum GitOp {
    CurrentBranch,
    Switch(String),
    Create(String),
    Merge(String),
}

#[derive(Debug)]
pub enum GitCause {
    Io(io::Error),
    Libgit(git2::Error),
    Utf8(FromUtf8Error),
    Failed(String),
}

#[derive(Debug)]
pub struct GitError {
    pub op: GitOp,
    pub cause: GitCause,
}

impl GitError {
    pub fn new(op: GitOp, cause: GitCause) -> Self {
        GitError { op, cause }
    }
}

pub(crate) fn with_op<T>(op: GitOp, f: impl FnOnce() -> Result<T, GitCause>) -> GitResult<T> {
    f().map_err(|cause| GitError::new(op, cause))
}

impl fmt::Display for GitOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitOp::CurrentBranch => write!(f, "Failed to get current branch"),
            GitOp::Switch(branch) => write!(f, "Failed to switch to branch '{}'", branch),
            GitOp::Create(branch) => write!(f, "Failed to create branch '{}'", branch),
            GitOp::Merge(branch) => write!(f, "Failed to merge branch '{}'", branch),
        }
    }
}

impl fmt::Display for GitCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitCause::Io(err) => write!(f, "could not run git: {}", err),
            GitCause::Libgit(err) => write!(f, "{}", err.message()),
            GitCause::Utf8(_) => write!(f, "git output was not valid UTF-8"),
            GitCause::Failed(message) => write!(f, "{}", message),
        }
    }
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.op, self.cause)
    }
}

impl Error for GitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.cause {
            GitCause::Io(err) => Some(err),
            GitCause::Libgit(err) => Some(err),
            GitCause::Utf8(err) => Some(err),
            GitCause::Failed(_) => None,
        }
    }
}

impl From<io::Error> for GitCause {
    fn from(err: io::Error) -> Self {
        GitCause::Io(err)
    }
}

impl From<git2::Error> for GitCause {
    fn from(err: git2::Error) -> Self {
        GitCause::Libgit(err)
    }
}

impl From<FromUtf8Error> for GitCause {
    fn from(err: FromUtf8Error) -> Self {
        GitCause::Utf8(err)
    }
}
