mod commands;
mod utils;

use commands::{branch_update, post_merge};
use std::env;
use utils::run_git;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match args_str.as_slice() {
        ["bu"] => branch_update(),
        ["ca"] => run_git(&["commit", "--amend"]),
        ["co"] => run_git(&["checkout"]),
        ["log"] => run_git(&["log", "--graph", "--oneline"]),
        ["pf"] => run_git(&["push", "--force-with-lease"]),
        ["pm"] => post_merge(),
        ["pr"] => run_git(&["pull", "--rebase"]),
        ["st"] => run_git(&["status"]),
        [] => run_git(&["--help"]),
        other => run_git(other),
    };
}
