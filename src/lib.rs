use std::path::PathBuf;

use git2::build::*;

#[no_mangle]
pub extern "C" fn do_stuff() {
    let _repo = RepoBuilder::new().clone("https://github.com/luketpeterson/metta-mod-catalog.git", &PathBuf::from("/tmp/repo/"));
}
