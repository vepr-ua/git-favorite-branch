mod harness;

use harness::{stderr_of, stdout_of, Sandbox};

const ORDERS: &str = "feat/JIRA-3413-connect-orders";
const ACCOUNTS: &str = "feat/JIRA-2443-account-creation-and-deletion";

#[test]
fn add_saves_current_branch_and_ls_prints_it() {
    let sandbox = Sandbox::new();
    sandbox.repo.checkout_new(ORDERS);

    sandbox
        .repo
        .gfb()
        .args(["add", "orders"])
        .assert()
        .success();

    let ls = sandbox.repo.gfb().args(["ls"]).assert().success();
    let stdout = stdout_of(&ls);
    assert!(
        stdout.contains(&format!("orders -> {ORDERS}")),
        "ls output:\n{stdout}"
    );
}

#[test]
fn new_creates_saved_branch_in_another_repo() {
    let sandbox = Sandbox::new();
    sandbox
        .repo
        .gfb()
        .args(["add", "orders", ORDERS])
        .assert()
        .success();

    let other = sandbox.another_repo();
    other.gfb().args(["new", "orders"]).assert().success();

    assert_eq!(other.current_branch(), ORDERS);
}

#[test]
fn use_checks_out_saved_branch() {
    let sandbox = Sandbox::new();
    sandbox.repo.create_branch(ORDERS);
    sandbox
        .repo
        .gfb()
        .args(["add", "orders", ORDERS])
        .assert()
        .success();

    sandbox
        .repo
        .gfb()
        .args(["use", "orders"])
        .assert()
        .success();

    assert_eq!(sandbox.repo.current_branch(), ORDERS);
}

#[test]
fn branch_prints_current_and_saved_name() {
    let sandbox = Sandbox::new();
    sandbox
        .repo
        .gfb()
        .args(["add", "orders", ORDERS])
        .assert()
        .success();

    let current = sandbox.repo.gfb().arg("branch").assert().success();
    assert_eq!(stdout_of(&current).trim(), "main");

    let saved = sandbox
        .repo
        .gfb()
        .args(["branch", "orders"])
        .assert()
        .success();
    assert_eq!(stdout_of(&saved).trim(), ORDERS);
}

#[test]
fn rename_moves_key() {
    let sandbox = Sandbox::new();
    sandbox
        .repo
        .gfb()
        .args(["add", "orders", ORDERS])
        .assert()
        .success();

    let renamed = sandbox
        .repo
        .gfb()
        .args(["rename", "orders", "checkout"])
        .assert()
        .success();
    let stdout = stdout_of(&renamed);
    assert!(
        stdout.contains("Renamed key from 'orders' to 'checkout'"),
        "rename output:\n{stdout}"
    );

    let saved = sandbox
        .repo
        .gfb()
        .args(["branch", "checkout"])
        .assert()
        .success();
    assert_eq!(stdout_of(&saved).trim(), ORDERS);
}

#[test]
fn del_removes_one_key_and_del_all_clears_the_rest() {
    let sandbox = Sandbox::new();
    sandbox
        .repo
        .gfb()
        .args(["add", "orders", ORDERS])
        .assert()
        .success();
    sandbox
        .repo
        .gfb()
        .args(["add", "accounts", ACCOUNTS])
        .assert()
        .success();

    sandbox
        .repo
        .gfb()
        .args(["del", "orders"])
        .assert()
        .success();

    let ls = sandbox.repo.gfb().args(["ls"]).assert().success();
    let stdout = stdout_of(&ls);
    assert!(
        stdout.contains(&format!("accounts -> {ACCOUNTS}")),
        "ls output:\n{stdout}"
    );
    assert!(
        !stdout.contains("orders ->"),
        "deleted key still listed:\n{stdout}"
    );

    sandbox.repo.gfb().arg("del-all").assert().success();

    let empty = sandbox.repo.gfb().args(["ls"]).assert().success();
    assert_eq!(stdout_of(&empty).trim(), "No favorite branches saved");
}

#[test]
fn sync_fast_forwards_current_branch() {
    let sandbox = Sandbox::new();
    sandbox.repo.checkout_new(ORDERS);
    sandbox.repo.commit_empty("orders ahead");
    sandbox.repo.checkout("main");

    sandbox
        .repo
        .gfb()
        .args(["add", "orders", ORDERS])
        .assert()
        .success();
    sandbox
        .repo
        .gfb()
        .args(["sync", "orders"])
        .assert()
        .success();

    assert_eq!(sandbox.repo.current_branch(), "main");
    assert_eq!(sandbox.repo.head_subject(), "orders ahead");
}

#[test]
fn use_missing_key_exits_with_error() {
    let sandbox = Sandbox::new();
    let failed = sandbox
        .repo
        .gfb()
        .args(["use", "missing"])
        .assert()
        .failure()
        .code(1);

    let stderr = stderr_of(&failed);
    assert!(
        stderr.contains("No branch saved for key 'missing'"),
        "stderr:\n{stderr}"
    );
}
