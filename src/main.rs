use git2::Repository;
use std::env;

fn main() {
    let path = match env::current_dir() {
        Ok(p) => p,
        Err(e) => panic!("failed to get path to current dir: {}", e),
    };

    let repo = match Repository::init(&path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    let diff = match repo.diff_index_to_workdir(None, None) {
        Ok(d) => d,
        Err(e) => panic!("failed to get diff: {}", e),
    };

    println!("{:?}", diff.get_delta(1))
}
