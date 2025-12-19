use std::process::{Command, Stdio};
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match args_str.as_slice() {
        [] => run_git(&["--help"]),
        ["bu"] => branch_update(),
        ["ca"] => run_git(&["commit", "--amend"]),
        ["co"] => run_git(&["checkout"]),
        ["log"] => run_git(&["log", "--graph", "--oneline"]),
        ["pf"] => run_git(&["push", "--force-with-lease"]),
        ["pr"] => run_git(&["pull", "--rebase"]),
        ["st"] => run_git(&["status"]),
        other => run_git(other),
    };
}

fn run_git(args: &[&str]) {
    let mut cmd = Command::new("git");
    let status = cmd.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status().expect("❌ Could not run git");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn branch_update() {
    let trunk = get_trunk();

    run_git(&["fetch", "-p"]);
    run_git(&["rebase", &format!("origin/{}", trunk)]);
    run_git(&["push", "--force-with-lease", "--no-verify"]);
}


fn get_trunk() -> String {
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null()) // silence les erreurs si non configuré
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if branch.starts_with("refs/remotes/origin/") {
                return branch.strip_prefix("refs/remotes/origin/").unwrap().to_string();
            }
        }
    }

    "master".to_string() // Fallback to "master" if unable to determine
}
