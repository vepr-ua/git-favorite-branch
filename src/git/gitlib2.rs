use crate::git::contract::GitProvider;

pub struct GitLib {}

impl GitProvider for GitLib {
    fn get_current_branch(&self) -> super::result::GitResult<String> {
        todo!()
    }

    fn switch_branch(&self, branch_name: &str) -> super::result::GitResult<()> {
        todo!()
    }

    fn create_branch(&self, branch_name: &str) -> super::result::GitResult<()> {
        todo!()
    }

    fn merge_branch(&self, branch_name: &str) -> super::result::GitResult<()> {
        todo!()
    }
}
