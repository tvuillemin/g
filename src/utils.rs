use std::process::{Command, Stdio};

pub fn get_trunk() -> String {
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null()) // silence les erreurs si non configuré
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if branch.starts_with("refs/remotes/origin/") {
                return branch
                    .strip_prefix("refs/remotes/origin/")
                    .unwrap()
                    .to_string();
            }
        }
    }

    "master".to_string() // Fallback to "master" if unable to determine
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
