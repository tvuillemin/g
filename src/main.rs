use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        run_git(&["--help"]);
        return;
    }

    let str_args: &[&str] = &args[1..].iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let git_args: Vec<&str> = match str_args {
        ["ca"] => vec!["commit", "--amend"],
        ["co"] => vec!["checkout"],
        ["st"] => vec!["status"],
        ["log"] => vec![
            "log",
            "--graph",
            "--oneline",
        ],

        other => other.to_vec(),
    };

    run_git(&git_args);
}

fn run_git(git_args: &[&str]) {
    let status = Command::new("git")
        .args(git_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("❌ Could not run `git`. Make sure Git is installed and in the PATH.");

    std::process::exit(status.code().unwrap_or(1));
}
