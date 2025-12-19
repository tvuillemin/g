use crate::utils::{get_trunk, run_git};

pub fn branch_update() {
    let trunk = get_trunk();
    run_git(&["fetch", "-p"]);
    run_git(&["rebase", &format!("origin/{}", trunk)]);
    run_git(&["push", "--force-with-lease", "--no-verify"]);
}
