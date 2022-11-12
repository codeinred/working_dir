use std::fs::{Metadata, ReadDir};
use std::io::Result;
use std::ops::Div;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct WorkingDir {
    path: PathBuf,
}

pub struct JoinResult {
    path: PathBuf,
}

impl AsRef<Path> for JoinResult {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl<P: AsRef<Path>> Div<P> for &WorkingDir {
    type Output = JoinResult;

    fn div(self, rhs: P) -> Self::Output {
        return JoinResult {
            path: self.path.join(rhs),
        };
    }
}

impl<P: AsRef<Path>> Div<P> for WorkingDir {
    type Output = JoinResult;

    fn div(self, rhs: P) -> Self::Output {
        return JoinResult {
            path: self.path.join(rhs),
        };
    }
}

impl WorkingDir {
    /// Create a WorkingDir
    pub fn new<P: AsRef<Path>>(path: P) -> WorkingDir {
        return WorkingDir {
            path: PathBuf::from(path.as_ref()),
        };
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> impl AsRef<Path> {
        self / path
    }

    /// Returns the canonical, absolute form of a path relative to the current working directory,
    /// with all intermediate components normalized and symbolic links resolved.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.canonicalize.html
    pub fn canonicalize<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        fs::canonicalize(self / path)
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
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<u64> {
        fs::copy(self.join(from), self.join(to))
    }

    /// Creates a new, empty directory at the provided path
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.create_dir.html
    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir(self / path)
    }

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
    pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir_all(self / path)
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
    pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(&self, original: P, link: Q) -> Result<()> {
        fs::hard_link(self / original, self / link)
    }

    /// Given a path, query the file system to get information about
    /// a file, directory, etc.
    ///
    /// This function will traverse symbolic links to query
    /// information about the destination file.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.metadata.html
    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        fs::metadata(self / path)
    }

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This is a convenience function for using `File::open` and
    /// `read_to_end` with fewer imports and without an intermediate variable.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read.html
    pub fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        fs::read(self / path)
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// The iterator will yield instances of `io::Result<DirEntry>`.
    /// New errors may be encountered after an iterator is initially
    /// constructed. Entries for the current and parent directories
    /// (typically `.` and `..`) are skipped.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    pub fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        fs::read_dir(self / path)
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_link.html
    pub fn read_link<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        fs::read_link(self / path)
    }

    /// Read the entire contents of a file into a string.
    ///
    /// This is a convenience function for using File::open
    /// and read_to_string with fewer imports and without an
    /// intermediate variable.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    pub fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        fs::read_to_string(self / path)
    }

    /// Removes an empty directory.
    ///
    /// See: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    pub fn remove_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::remove_dir(self / path)
    }
}

#[cfg(test)]
mod tests;
