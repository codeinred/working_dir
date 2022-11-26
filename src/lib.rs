use core::fmt::Debug;
use std::fs::{File, Metadata, OpenOptions, ReadDir};
use std::io::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub use path_no_alloc::with_paths;

fn create_parents<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
    } else {
        Ok(())
    }
}

impl <P> Dir<P> where P: AsRef<Path>
{
    /// Join this working dir with some path
    pub fn join<P2: AsRef<Path>>(&self, path: P2) -> PathBuf {
        self.as_ref().join(path)
    }

    /// Opens a file with the given OpenOptions
    pub fn open<P2: AsRef<Path>>(&self, path: P2, opts: &OpenOptions) -> Result<File> {
        with_paths! { path = self / path => opts.open(path) }
    }

    /// Opens a file in read-only mode
    ///
    /// See: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
    pub fn open_readonly<P2: AsRef<Path>>(&self, path: P2) -> Result<File> {
        with_paths! { path = self / path => File::open(path) }
    }

    /// Creates any parent directories for a given path. Does nothing
    /// if the path has no parents.
    ///
    /// # Errors
    /// This function returns an error if the creation of the parent
    /// directories fails
    pub fn create_parents<P2: AsRef<Path>>(&self, path: P2) -> Result<()> {
        with_paths! {
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
    pub fn exists<P2: AsRef<Path>>(&self, path: P2) -> bool {
        with_paths! { path = self / path => path.exists() }
    }

    #[inline]
    /// Checks if this directory contains a given path. Alias for [exists()](#method.exists)
    pub fn contains<P2: AsRef<Path>>(&self, path: P2) -> bool {
        self.exists(path)
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
    pub fn try_exists<P2: AsRef<Path>>(&self, path: P2) -> Result<bool> {
        with_paths! { path = self / path => path.try_exists() }
    }

    #[inline]
    /// Checks if this directory contains a given path. Alias for [try_exists()](#method.try_exists)
    pub fn try_contains<P2: AsRef<Path>>(&self, path: P2) -> Result<bool> {
        self.try_exists(path)
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
    pub fn move_to<P2: AsRef<Path>, P3: AsRef<Path>>(&self, new_root: P2, path: P3) -> Result<()> {
        let path = path.as_ref();
        with_paths! {
            old_path = self / path,
            new_path = new_root / path
        }
        create_parents(new_path)?;
        fs::rename(old_path, new_path)
    }

    /// Returns the canonical, absolute form of a path relative to the current working directory,
    /// with all intermediate components normalized and symbolic links resolved.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.canonicalize.html
    pub fn canonicalize<P2: AsRef<Path>>(&self, path: P2) -> Result<PathBuf> {
        with_paths! { path = self / path => fs::canonicalize(path) }
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
    pub fn copy<P2: AsRef<Path>, P3: AsRef<Path>>(&self, from: P2, to: P3) -> Result<u64> {
        with_paths! {
            from = self / from,
            to = self / to
        }
        fs::copy(from, to)
    }

    /// Creates a new, empty directory at the provided path
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    pub fn create_dir<P2: AsRef<Path>>(&self, path: P2) -> Result<()> {
        with_paths! { path = self / path => fs::create_dir(path) }
    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    pub fn create_dir_all<P2: AsRef<Path>>(&self, path: P2) -> Result<()> {
        with_paths! { path = self / path => fs::create_dir_all(path) }
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
    pub fn hard_link<P2: AsRef<Path>, P3: AsRef<Path>>(&self, original: P2, link: P3) -> Result<()> {
        with_paths! {
            original = self / original,
            link = self / link
        }
        fs::hard_link(original, link)
    }

    /// Given a path, query the file system to get information about
    /// a file, directory, etc.
    ///
    /// This function will traverse symbolic links to query
    /// information about the destination file.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.metadata.html
    pub fn metadata<P2: AsRef<Path>>(&self, path: P2) -> Result<Metadata> {
        with_paths! { path = self / path => fs::metadata(path) }
    }

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This is a convenience function for using `File::open` and
    /// `read_to_end` with fewer imports and without an intermediate variable.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read.html
    pub fn read<P2: AsRef<Path>>(&self, path: P2) -> Result<Vec<u8>> {
        with_paths! { path = self / path => fs::read(path) }
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// The iterator will yield instances of `io::Result<DirEntry>`.
    /// New errors may be encountered after an iterator is initially
    /// constructed. Entries for the current and parent directories
    /// (typically `.` and `..`) are skipped.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    pub fn read_dir<P2: AsRef<Path>>(&self, path: P2) -> Result<ReadDir> {
        with_paths! { path = self / path => fs::read_dir(path) }
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_link.html
    pub fn read_link<P2: AsRef<Path>>(&self, path: P2) -> Result<PathBuf> {
        with_paths! { path = self / path => fs::read_link(path) }
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This is a convenience function for using File::open
    /// and read_to_string with fewer imports and without an
    /// intermediate variable.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    pub fn read_to_string<P2: AsRef<Path>>(&self, path: P2) -> Result<String> {
        with_paths! { path = self / path => fs::read_to_string(path) }
    }

    /// Removes an empty directory.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    pub fn remove_dir<P2: AsRef<Path>>(&self, path: P2) -> Result<()> {
        with_paths! { path = self / path => fs::remove_dir(path) }
    }

    /// Removes a directory at this path, after removing all
    /// its contents. Use carefully!
    ///
    /// This function does **not** follow symbolic links and it will
    /// simply remove the symbolic link itself.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html
    pub fn remove_dir_all<P2: AsRef<Path>>(&self, path: P2) -> Result<()> {
        with_paths! { path = self / path => fs::remove_dir_all(path) }
    }

    /// Removes a file from the filesystem.
    ///
    /// Note that there is no guarantee that the file is immediately
    /// deleted (e.g., depending on platform, other open file
    /// descriptors may prevent immediate removal).
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.remove_file.html
    pub fn remove_file<P2: AsRef<Path>>(&self, path: P2) -> Result<()> {
        with_paths! { path = self / path => fs::remove_file(path) }
    }

    /// Rename a file or directory to a new name, replacing
    /// the original file if to already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.rename.html
    pub fn rename<P2: AsRef<Path>, P3: AsRef<Path>>(&self, from: P2, to: P3) -> Result<()> {
        with_paths! {
            from = self / from,
            to = self / to
        }
        fs::rename(from, to)
    }

    /// Query the metadata about a file without following symlinks.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html
    pub fn symlink_metadata<P2: AsRef<Path>>(&self, path: P2) -> Result<Metadata> {
        with_paths! { path = self / path => fs::symlink_metadata(path) }
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
    pub fn write<P2: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P2, contents: C) -> Result<()> {
        with_paths! {
            path = self / path => fs::write(path, contents)
        }
    }
}

#[repr(transparent)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Dir<P>
where
    P: AsRef<Path>,
{
    path: P,
}

impl<P> Debug for Dir<P>
where
    P: AsRef<Path>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.as_ref();

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

impl<P> From<P> for Dir<P>
where
    P: AsRef<Path>,
{
    #[inline]
    fn from(value: P) -> Self {
        Self::new(value)
    }
}

impl<P> Dir<P>
where
    P: AsRef<Path>,
{
    #[inline]
    pub fn new(path: P) -> Dir<P> {
        Dir { path: path.into() }
    }
}

impl<P> AsRef<Path> for Dir<P>
where
    P: AsRef<Path>,
{
    #[inline]
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

#[cfg(test)]
mod tests;
