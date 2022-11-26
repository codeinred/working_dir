use std::path::{Path, PathBuf};

use working_dir::Dir;

fn find_include<P: AsRef<Path>, Q: AsRef<Path>>(include_set: &[Dir<P>], file: Q) -> Option<PathBuf> {
    let file = file.as_ref();

    for include_dir in include_set {
        if include_dir.contains(file) {
            return Some(include_dir / file)
        }
    }
    return None
}


fn main() {
    let file = "stdio.h";
    let include_set = [
        "/usr/local/include",
        "/usr/target/include",
        "/usr/include"
    ].map(Dir);

    if let Some(path) = find_include(&include_set, file) {

        println!("{file} found at {path:?}")
    } else {
        println!("Unable to find {file} in {include_set:?}")
    }
}
