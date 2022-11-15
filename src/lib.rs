use core::fmt::Debug;
use std::fs::{File, Metadata, OpenOptions, ReadDir};
use std::io::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod with_joined;

fn create_parents<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
    } else {
        Ok(())
    }
}

pub trait WorkingDir
where
    Self: AsRef<Path>,
{
    /// Join this working dir with some path
    fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.as_ref().join(path)
    }

    /// Opens a file with the given OpenOptions
    fn open<P: AsRef<Path>>(&self, path: P, opts: &OpenOptions) -> Result<File> {
        with_joined! { path = self / path => opts.open(path) }
    }

    /// Opens a file in read-only mode
    ///
    /// See: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
    fn open_readonly<P: AsRef<Path>>(&self, path: P) -> Result<File> {
        with_joined! { path = self / path => File::open(path) }
    }

    /// Creates any parent directories for a given path. Does nothing
    /// if the path has no parents.
    ///
    /// # Errors
    /// This function returns an error if the creation of the parent
    /// directories fails
    fn create_parents<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        with_joined! {
            path = self / path => create_parents(path)
        }
    }

    /// Returns true if the path points at an existing entity.
    ///
    /// Warning: this method may be error-prone, consider using try_exists()
    /// instead! It also has a risk of introducing time-of-check to
    /// time-of-use (TOCTOU) bugs.
    ///
    /// This function will traverse symbolic links to query information
    /// about the destination file.
    ///
    /// If you cannot access the metadata of the file, e.g. because of
    /// a permission error or broken symbolic links, this will return false.
    ///
    /// See: https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.exists
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        with_joined! { path = self / path => path.exists() }
    }

    /// Returns `Ok(true)` if the path points at an existing entity.
    ///
    /// This function will traverse symbolic links to query information
    /// about the destination file. In case of broken symbolic links this
    /// will return `Ok(false)`.
    ///
    /// As opposed to the `exists()` method, this one doesn’t silently
    /// ignore errors unrelated to the path not existing. (E.g. it will
    /// return `Err(_)` in case of permission denied on some of the parent
    /// directories.)
    ///
    /// Note that while this avoids some pitfalls of the `exists()` method,
    /// it still can not prevent time-of-check to time-of-use (TOCTOU) bugs.
    /// You should only use it in scenarios where those bugs are not an issue.
    ///
    /// See: https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.try_exists
    fn try_exists<P: AsRef<Path>>(&self, path: P) -> Result<bool> {
        with_joined! { path = self / path => path.try_exists() }
    }

    /// Moves a path from this working directory, to another working directory.
    ///
    /// Suppose we have some path `path/to/thing`, corresponding to `<self>/path/to/thing`
    /// in the current working directory.
    ///
    /// This function will move it to `<B>/path/to/thing`, in working directory B, creating
    /// any parent dirs as necessary
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    ///
    /// - `<working dir>/<path>` does not exist (in which case, there is nothing to rename)
    /// - The user lacks permission to view the contents of the path.
    /// - The destination is on a separate filesystem
    fn move_to<WD: WorkingDir, P: AsRef<Path>>(&self, new_root: WD, path: P) -> Result<()> {
        let path = path.as_ref();
        with_joined! {
            old_path = self / path,
            new_path = new_root / path
            =>
            create_parents(new_path)?;
            fs::rename(old_path, new_path)
        }
    }

    /// Returns the canonical, absolute form of a path relative to the current working directory,
    /// with all intermediate components normalized and symbolic links resolved.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.canonicalize.html
    fn canonicalize<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        with_joined! { path = self / path => fs::canonicalize(path) }
    }

    /// Copies the contents of one file to another. This function
    /// will also copy the permission bits of the original file to the
    /// destination file.
    ///
    /// This function will **overwrite** the contents of to.
    ///
    /// Note that if from and to both point to the same file, then
    /// the file will likely get truncated by this operation.
    ///
    /// On success, the total number of bytes copied is returned and it
    /// is equal to the length of the to file as reported by metadata.
    ///
    /// If you’re wanting to copy the contents of one file to another
    /// and you’re working with Files, see the io::copy() function.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<u64> {
        with_joined! {
            from = self / from,
            to = self / to
            => fs::copy(from, to)
        }
    }

    /// Creates a new, empty directory at the provided path
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        with_joined! { path = self / path => fs::create_dir(path) }
    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        with_joined! { path = self / path => fs::create_dir_all(path) }
    }

    /// Creates a new hard link on the filesystem.
    ///
    /// The link path will be a link pointing to the original path.
    /// Note that systems often require these two paths to both be
    /// located on the same filesystem.
    ///
    /// If original names a symbolic link, it is platform-specific
    /// whether the symbolic link is followed. On platforms where
    /// it’s possible to not follow it, it is not followed, and the
    /// created hard link points to the symbolic link itself.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.hard_link.html
    fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(&self, original: P, link: Q) -> Result<()> {
        with_joined! {
            original = self / original,
            link = self / link
            => fs::hard_link(original, link)
        }
    }

    /// Given a path, query the file system to get information about
    /// a file, directory, etc.
    ///
    /// This function will traverse symbolic links to query
    /// information about the destination file.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.metadata.html
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        with_joined! { path = self / path => fs::metadata(path) }
    }

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This is a convenience function for using `File::open` and
    /// `read_to_end` with fewer imports and without an intermediate variable.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read.html
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        with_joined! { path = self / path => fs::read(path) }
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// The iterator will yield instances of `io::Result<DirEntry>`.
    /// New errors may be encountered after an iterator is initially
    /// constructed. Entries for the current and parent directories
    /// (typically `.` and `..`) are skipped.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        with_joined! { path = self / path => fs::read_dir(path) }
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_link.html
    fn read_link<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        with_joined! { path = self / path => fs::read_link(path) }
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This is a convenience function for using File::open
    /// and read_to_string with fewer imports and without an
    /// intermediate variable.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        with_joined! { path = self / path => fs::read_to_string(path) }
    }

    /// Removes an empty directory.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    fn remove_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        with_joined! { path = self / path => fs::remove_dir(path) }
    }

    /// Removes a directory at this path, after removing all
    /// its contents. Use carefully!
    ///
    /// This function does **not** follow symbolic links and it will
    /// simply remove the symbolic link itself.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
    fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        with_joined! { path = self / path => fs::remove_dir_all(path) }
    }

    /// Removes a file from the filesystem.
    ///
    /// Note that there is no guarantee that the file is immediately
    /// deleted (e.g., depending on platform, other open file
    /// descriptors may prevent immediate removal).
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.remove_file.html
    fn remove_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        with_joined! { path = self / path => fs::remove_file(path) }
    }

    /// Rename a file or directory to a new name, replacing
    /// the original file if to already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.rename.html
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()> {
        with_joined! {
            from = self / from,
            to = self / to
            => fs::rename(from, to)
        }
    }

    /// Query the metadata about a file without following symlinks.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html
    fn symlink_metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        with_joined! { path = self / path => fs::symlink_metadata(path) }
    }

    /// Write a slice as the entire contents of a file.
    ///
    /// This function will create a file if it does not exist, and will
    /// entirely replace its contents if it does.
    ///
    /// Depending on the platform, this function may fail if the full
    /// directory path does not exist.
    ///
    /// This is a convenience function for using File::create and
    /// write_all with fewer imports.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.write.html
    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, contents: C) -> Result<()> {
        with_joined! {
            path = self / path => fs::write(path, contents)
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Dir {
    path: PathBuf,
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.as_path();

        if let Some(path) = path.to_str() {
            if path.ends_with("/") {
                write!(f, "Dir(\"{path}\")")
            } else {
                write!(f, "Dir(\"{path}/\")")
            }
        } else {
            let path = path.to_string_lossy();
            if path.ends_with("/") {
                write!(f, "Dir(\"{path}\")")
            } else {
                write!(f, "Dir(\"{path}/\")")
            }
        }
    }
}

impl Dir {
    pub fn new<P: Into<PathBuf>>(path: P) -> Dir {
        Dir { path: path.into() }
    }
}

impl AsRef<Path> for Dir {
    fn as_ref(&self) -> &Path {
        self.path.as_path()
    }
}

impl<P: AsRef<Path>> WorkingDir for P {}

#[cfg(test)]
mod tests;
