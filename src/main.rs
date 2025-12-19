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
    run_git(&["fetch", "-p"]);
    run_git(&["rebase", "origin/master"]);
    run_git(&["push", "--force-with-lease", "--no-verify"]);
}
