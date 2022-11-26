# `working_dir`: Working Directories as local state

Working directories and relative paths are great, but a working directory is
global state. This is no good. Changing the working directory of the whole
program is such a hack, and there are plenty of times you want to do something
in the context _of_ a working directory other than the one you're currently
running in.

This is a small library intended to provide objects that act as a working
directory, supporting a full range of file and directory manipulations. It was
created as part of a larger project to create a tool to automatically handle
large-scale refactors of C and C++ projects.

When refactoring such a project, it's necessary to update both the locations of
files, and any `#include` declarations that pertain to moved files. In such a
context, it's useful to think of an inlcude set as a list of as working
directories, and the project's root directory can also be thought of as a
working directory that files are being moved within.

Here's a minimal example of one way this library might be used in such a
context:

```rust
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
```
