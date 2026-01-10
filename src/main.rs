mod commands;
mod utils;

use commands::{branch_update, post_merge};
use std::env;
use utils::{get_trunk, run_git};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        run_git(&["--help"]);
        return;
    }

    let (main_arg, extra_args) = args.split_first().unwrap();

    match main_arg.as_str() {
        "bu" => branch_update(),
        "ca" => run_git(&["commit", "--amend"]),
        "co" => {
            let mut git_args = vec!["checkout"];
            git_args.extend(extra_args.iter().map(|s| s.as_str()));
            run_git(&git_args);
        }
        "log" => run_git(&["log", "--graph", "--oneline"]),
        "pf" => run_git(&["push", "--force-with-lease"]),
        "pm" => post_merge(),
        "pr" => run_git(&["pull", "--rebase"]),
        "st" => run_git(&["status"]),
        "tr" => run_git(&["switch", get_trunk().as_str()]),
        _ => {
            let mut git_args = vec![main_arg.as_str()];
            git_args.extend(extra_args.iter().map(|s| s.as_str()));
            run_git(&git_args);
        }
    };
}
