use std::process::{Command, Stdio};

pub fn get_current_branch() -> String {
    let cmd = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .unwrap();

    let current_branch = String::from_utf8_lossy(&cmd.stdout).trim().to_string();

    current_branch
}

pub fn get_trunk() -> String {
    let cmd = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .unwrap();

    let branch_full_name = String::from_utf8_lossy(&cmd.stdout).trim().to_string();

    branch_full_name
        .strip_prefix("refs/remotes/origin/")
        .unwrap()
        .to_string()
}

pub fn run_git(args: &[&str]) {
    let mut cmd = Command::new("git");
    let status = cmd
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("❌ Could not run git");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
