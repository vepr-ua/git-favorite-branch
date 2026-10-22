use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command as StdCommand, Output};

use assert_cmd::Command;
use tempfile::TempDir;

pub struct Sandbox {
    home: TempDir,
    pub repo: Repo,
}

impl Sandbox {
    pub fn new() -> Self {
        let home = TempDir::new().expect("create temp home");
        let repo = Repo::init(home.path());
        Self { home, repo }
    }

    pub fn another_repo(&self) -> Repo {
        Repo::init(self.home.path())
    }
}

pub struct Repo {
    home: PathBuf,
    dir: TempDir,
}

impl Repo {
    pub fn init(home: &Path) -> Self {
        let dir = TempDir::new().expect("create temp repo");
        let repo = Self {
            home: home.to_path_buf(),
            dir,
        };
        repo.git(["init", "-b", "main"]);
        repo.commit_empty("initial");
        repo
    }

    pub fn gfb(&self) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_gfb"));
        cmd.current_dir(self.dir.path())
            .env("HOME", &self.home)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE");
        cmd
    }

    pub fn current_branch(&self) -> String {
        let output = self.git(["branch", "--show-current"]);
        String::from_utf8(output.stdout)
            .expect("branch name is utf-8")
            .trim()
            .to_string()
    }

    pub fn head_subject(&self) -> String {
        let output = self.git(["log", "-1", "--format=%s"]);
        String::from_utf8(output.stdout)
            .expect("commit subject is utf-8")
            .trim()
            .to_string()
    }

    pub fn checkout(&self, branch: &str) {
        self.git(["checkout", branch]);
    }

    pub fn checkout_new(&self, branch: &str) {
        self.git(["checkout", "-b", branch]);
    }

    pub fn create_branch(&self, branch: &str) {
        self.git(["branch", branch]);
    }

    pub fn commit_empty(&self, message: &str) {
        self.git(["commit", "--allow-empty", "-m", message]);
    }

    fn git<I, S>(&self, args: I) -> Output
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args: Vec<S> = args.into_iter().collect();
        let display: Vec<&str> = args
            .iter()
            .map(|arg| arg.as_ref().to_str().unwrap_or("?"))
            .collect();

        let output = StdCommand::new("git")
            .args([
                "-c",
                "user.email=e2e@example.com",
                "-c",
                "user.name=e2e",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(&args)
            .current_dir(self.dir.path())
            .env("HOME", &self.home)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .output()
            .unwrap_or_else(|err| panic!("failed to spawn git {display:?}: {err}"));

        if !output.status.success() {
            panic!(
                "git {display:?} exited {}\nstdout:\n{}\nstderr:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        output
    }
}

pub fn stdout_of(assertion: &assert_cmd::assert::Assert) -> String {
    String::from_utf8(assertion.get_output().stdout.clone()).expect("stdout is utf-8")
}

pub fn stderr_of(assertion: &assert_cmd::assert::Assert) -> String {
    String::from_utf8(assertion.get_output().stderr.clone()).expect("stderr is utf-8")
}
