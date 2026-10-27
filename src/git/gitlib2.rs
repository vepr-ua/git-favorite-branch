use crate::git::{contract::GitProvider, error::GitError, result::GitResult};
use git2::{build::CheckoutBuilder, AnnotatedCommit, Repository, RepositoryState};

pub struct GitLib {
    repo: git2::Repository,
}

impl GitLib {
    pub fn new_with_repo(path: Option<&str>) -> GitResult<Self> {
        let repo_path = path.unwrap_or(".");
        let repo = Repository::discover(repo_path)?;

        Ok(GitLib { repo })
    }

    fn fast_forward(&self, their: &AnnotatedCommit) -> GitResult<()> {
        let target = self.repo.find_commit(their.id())?;

        let mut opts = CheckoutBuilder::new();
        opts.safe();
        self.repo
            .checkout_tree(target.as_object(), Some(&mut opts))?;

        let mut head_ref = self.repo.head()?;
        head_ref.set_target(target.id(), "gfb sync: fast forward")?;
        Ok(())
    }

    fn normal_merge(&self, branch_name: &str, their: &AnnotatedCommit) -> GitResult<()> {
        self.repo.merge(&[&their], None, None)?;

        let mut index = self.repo.index()?;
        if index.has_conflicts() {
            return Err(GitError::Failed(
                "merge has conflicts; resolve them and commit".to_owned(),
            ));
        }

        let tree = self.repo.find_tree(index.write_tree()?)?;
        let sig = self.repo.signature()?;
        let ours = self.repo.head()?.peel_to_commit()?;
        let theirs = self.repo.find_commit(their.id())?;

        self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &format!("Merge branch '{branch_name}'"),
            &tree,
            &[&ours, &theirs],
        )?;

        self.repo.cleanup_state()?;
        Ok(())
    }
}

impl GitProvider for GitLib {
    fn get_current_branch(&self) -> GitResult<String> {
        let head_ref = self.repo.head()?;
        let ref_name = head_ref.shorthand()?;
        Ok(ref_name.to_owned())
    }

    fn switch_branch(&self, branch_name: &str) -> GitResult<()> {
        let obj = self.repo.revparse_single(branch_name)?;
        let mut opt = CheckoutBuilder::new();
        opt.safe();

        self.repo.checkout_tree(&obj, Some(&mut opt))?;
        self.repo.set_head(&format!("refs/heads/{branch_name}"))?;
        Ok(())
    }

    fn create_branch(&self, branch_name: &str) -> GitResult<()> {
        let head_ref = self.repo.head()?;
        let head_commit = head_ref.peel_to_commit()?;
        let branch = self.repo.branch(branch_name, &head_commit, false)?;

        let Ok(ref_name) = branch.get().name() else {
            return Err(GitError::Failed(
                "branch name is not valid UTF-8".to_owned(),
            ));
        };

        self.repo.set_head(ref_name)?;
        Ok(())
    }

    fn merge_branch(&self, branch_name: &str) -> GitResult<()> {
        if self.repo.state() != RepositoryState::Clean {
            return Err(GitError::Failed(
                "repository is in the middle of another operation".to_owned(),
            ));
        }

        let branch = self
            .repo
            .find_branch(branch_name, git2::BranchType::Local)?;
        let their = self.repo.reference_to_annotated_commit(branch.get())?;

        let (analysis, _preference) = self.repo.merge_analysis(&[&their])?;

        if analysis.is_up_to_date() {
            return Ok(());
        }

        if analysis.is_fast_forward() {
            return self.fast_forward(&their);
        }

        if analysis.is_normal() {
            return self.normal_merge(branch_name, &their);
        }

        Err(GitError::Failed("nothing to merge".to_owned()))
    }
}
