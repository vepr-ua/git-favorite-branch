use crate::{
    config::Config,
    errors::{GFBError, Result},
    git_helpers::{create_branch, switch_branch},
};
use std::collections::HashMap;

pub struct CommandManager {
    pub config: Config,
}

impl CommandManager {
    pub fn add_branch(&mut self, key: String, branch_name: String) -> Result<()> {
        self.config.state.insert(key, branch_name);
        self.config.save()
    }

    pub fn switch_to_branch(&self, key: String) -> Result<()> {
        let Some(branch_name) = self.config.state.get(&key) else {
            return Err(GFBError::KeyNotFound(key))?;
        };

        switch_branch(branch_name)?;

        Ok(())
    }

    pub fn delete_branch(&mut self, key: String) -> Result<()> {
        let Some(_) = self.config.state.remove(&key) else {
            return Err(GFBError::KeyNotFound(key));
        };

        self.config.save()
    }

    pub fn create_new_branch(&self, key: String) -> Result<()> {
        let branch_name = self
            .config
            .state
            .get(&key)
            .ok_or_else(|| GFBError::KeyNotFound(key))?;

        create_branch(branch_name)?;
        Ok(())
    }

    pub fn list_branches(&self) -> Option<&HashMap<String, String>> {
        let ref state = self.config.state;
        let state_iter = state.into_iter();
        if state_iter.len().eq(&0) {
            return None;
        }

        Some(state)
    }

    pub fn clear_branches(&mut self) -> Result<()> {
        self.config.state = HashMap::new();
        self.config.save()
    }

    pub fn print_branch_name(&self, key: String) -> Result<String> {
        let Some(branch_name) = self.config.state.get(&key) else {
            return Err(GFBError::KeyNotFound(key));
        };

        Ok(branch_name.to_string())
    }
}
