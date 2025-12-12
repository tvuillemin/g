use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        run_git(&["--help".to_string()]);
        return;
    }

    let git_args = &args[1..];

    run_git(git_args);
}

fn run_git(git_args: &[String]) {
    let status = Command::new("git")
        .args(git_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("❌ Could not run `git`. Make sure Git is installed and in the PATH.");

    std::process::exit(status.code().unwrap_or(1));
}
