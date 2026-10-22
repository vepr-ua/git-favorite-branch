use crate::git::error::GitError;

pub type GitResult<T> = std::result::Result<T, GitError>;
