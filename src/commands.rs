use crate::utils::{get_current_branch, get_trunk, run_git};

pub fn branch_update() {
    let trunk = get_trunk();
    run_git(&["fetch", "-p"]);
    run_git(&["rebase", &format!("origin/{}", trunk)]);
    run_git(&["push", "--force-with-lease", "--no-verify"]);
}

pub fn post_merge() {
    let branch_to_be_deleted = get_current_branch();
    let trunk = get_trunk();

    run_git(&["checkout", &trunk]);
    run_git(&["pull", "--rebase"]);
    run_git(&["branch", "-d", &branch_to_be_deleted]);
}
