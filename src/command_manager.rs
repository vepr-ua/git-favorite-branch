use crate::{
    config::Config,
    errors::{GFBError, Result},
    git_helpers::{create_branch, get_current_branch, merge_branch, switch_branch},
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
            return Err(GFBError::KeyNotFound(Some(key)))?;
        };

        switch_branch(branch_name)?;

        Ok(())
    }

    pub fn delete_branch(&mut self, key: String) -> Result<()> {
        let Some(_) = self.config.state.remove(&key) else {
            return Err(GFBError::KeyNotFound(Some(key)));
        };

        self.config.save()
    }

    pub fn create_new_branch(&self, key: String) -> Result<()> {
        let branch_name = self
            .config
            .state
            .get(&key)
            .ok_or_else(|| GFBError::KeyNotFound(Some(key)))?;

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
            return Err(GFBError::KeyNotFound(Some(key)));
        };

        Ok(branch_name.to_string())
    }

    pub fn rename_key(&mut self, new_key: &str, old_key: Option<&str>) -> Result<(String, String)> {
        let old_key = match old_key {
            Some(key) => key.to_owned(),
            None => self.infer_key_from_branch()?,
        };

        if !self.config.state.contains_key(&old_key) {
            return Err(GFBError::KeyNotFound(Some(old_key)));
        }

        if old_key == new_key {
            return Ok((old_key, new_key.to_owned()));
        }

        if self.config.state.contains_key(new_key) {
            return Err(GFBError::KeyAlreadyExists(new_key.to_string()));
        }

        let value = self
            .config
            .state
            .remove(&old_key)
            .ok_or(GFBError::KeyNotFound(Some(old_key.to_owned())))?;

        self.config.state.insert(new_key.to_string(), value);
        self.config.save()?;

        Ok((old_key.to_owned(), new_key.to_owned()))
    }

    fn infer_key_from_branch(&self) -> Result<String> {
        let current_branch = get_current_branch()?;
        let current_branch = current_branch.trim();

        let branch_matches: Vec<&str> = self
            .config
            .state
            .iter()
            .filter(|(_, value)| value.as_str() == current_branch)
            .map(|(key, _)| key.as_str())
            .collect();

        match branch_matches.as_slice() {
            [key] => Ok((*key).to_owned()),
            [] => Err(GFBError::KeyNotFound(None)),
            _ => Err(GFBError::AmbiguousKey()),
        }
    }

    pub fn sync_branch(&self, key: &str) -> Result<()> {
        let Some(found_branch) = self.config.state.get(key) else {
            return Err(GFBError::KeyNotFound(Some(key.to_owned())));
        };

        merge_branch(found_branch)?;

        Ok(())
    }
}
