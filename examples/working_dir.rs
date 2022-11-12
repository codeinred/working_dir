use std::{io::Result, path::Path};
use working_dir::{Dir, WorkingDir};

fn main() -> Result<()> {
    let cwd = Dir::new("my/root");
    let path = Path::new("path/to/file.txt");

    cwd.create_parents(path)?;

    cwd.write(path, "Hello, world!\n")?;

    // This path *shouldn't* exist in the program's global working directory
    assert!(!path.exists());

    // But it *should* exist in the directory specified by cwd
    assert!(cwd.exists(path));

    // This should be the same as the previous assert
    assert!(Path::new("my/root/path/to/file.txt").exists());

    // Check that the content is what we expect
    assert_eq!(cwd.read_to_string(path)?, "Hello, world!\n");

    let other_cwd = Dir::new("some/other/root");

    cwd.move_to(&other_cwd, path)?;

    // Now we should exist in the other directory
    assert!(!cwd.exists(path));
    assert!(other_cwd.exists(path));

    Ok(())
}
