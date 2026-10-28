use crate::git::result::GitResult;

pub trait GitProvider {
    fn get_current_branch(&self) -> GitResult<String>;
    fn switch_branch(&self, branch_name: &str) -> GitResult<()>;
    fn create_branch(&self, branch_name: &str) -> GitResult<()>;
    fn merge_branch(&self, branch_name: &str) -> GitResult<()>;
}
