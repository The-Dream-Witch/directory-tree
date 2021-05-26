//! Directory Tree Simulator: Provides a directory tree structure and an operating system stub
//! structure to interact with it.

// Bart Massey 2021

// Workaround for Clippy false positive in Rust 1.51.0.
// https://github.com/rust-lang/rust-clippy/issues/6546
#![allow(clippy::result_unit_err)]

use thiserror::Error;

/// Errors during directory interaction.
#[derive(Error, Debug)]
pub enum DirError<'a> {
    /// The character `/` in component names is disallowed,
    /// to make path separators easier.
    #[error("{0}: slash in name is invalid")]
    SlashInName(&'a str),
    /// Only one subdirectory of a given name can exist in any directory.
    #[error("{0}: directory exists")]
    DirExists(&'a str),
    /// Traversal failed due to missing subdirectory.
    #[error("{0}: invalid element in path")]
    InvalidChild(&'a str),
}

/// Result type for directory errors.
pub type Result<'a, T> = std::result::Result<T, DirError<'a>>;

/// A directory entry. Component names are stored externally.
#[derive(Debug, Clone)]
pub struct DEnt<'a> {
    pub name: &'a str,
    pub subdir: DTree<'a>,
}

/// A directory tree.
#[derive(Debug, Clone, Default)]
pub struct DTree<'a> {
    pub children: Vec<DEnt<'a>>,
}

/// Operating system state: the directory tree and the current working directory.
#[derive(Debug, Clone, Default)]
pub struct OsState<'a> {
    pub dtree: DTree<'a>,
    pub cwd: Vec<&'a str>,
}

impl<'a> DEnt<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        Ok(Self {
            name,
            subdir: DTree::new(),
        })
    }

    fn paths(&self) -> Vec<String> {
        let mut pathvec: Vec<String> = Vec::new();

        if !self.subdir.children.is_empty() {
            for x in &self.subdir.children {
                for y in x.paths() {
                    pathvec.push(self.name.to_string() + "/" + &y);
                }
            }
        } else {
            pathvec.push(self.name.to_string() + "/");
        }

        pathvec
    }
}

impl<'a> DTree<'a> {
    /// Create a new empty directory tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Make a subdirectory with the given name in this directory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("test").unwrap();
    /// assert_eq!(&dt.paths(), &["/test/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::SlashInName` if `name` contains `/`.
    /// * `DirError::DirExists` if `name` already exists.
    pub fn mkdir(&mut self, name: &'a str) -> Result<()> {
        if name.contains('/') {
            return Err(DirError::SlashInName(name));
        }

        for x in &self.children {
            if x.name == name {
                return Err(DirError::DirExists(name));
            }
        }

        let entry = DEnt::new(name).unwrap();
        self.children.push(entry);
        Ok(())
    }

    /// Traverse to the subdirectory given by `path` and then call `f` to visit the subdirectory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("test").unwrap();
    /// let paths = dt.with_subdir(&["test"], |dt| dt.paths()).unwrap();
    /// assert_eq!(&paths, &["/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if `path` is invalid.
    pub fn with_subdir<'b, F, R>(&'b self, path: &[&'a str], f: F) -> Result<R>
    where
        F: FnOnce(&'b DTree<'a>) -> R,
    {
        if path.is_empty() {
            return Err(DirError::InvalidChild(""));
        }

        let paths: Vec<&'a str> = path.iter().rev().cloned().collect();
        self.subdir(paths, f)
    }

    pub fn subdir<'b, F, R>(&'b self, mut path: Vec<&'a str>, f: F) -> Result<R>
    where
        F: FnOnce(&'b DTree<'a>) -> R,
    {
        if path.is_empty() {
            return Ok(f(self));
        }

        let name = path.pop().unwrap();
        for x in &self.children {
            if x.name == name {
                return x.subdir.subdir(path, f);
            }
        }

        Err(DirError::InvalidChild(name))
    }

    /// Traverse to the subdirectory given by `path` and then call `f` to visit the subdirectory
    /// mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("a").unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("b").unwrap()).unwrap();
    /// assert_eq!(&dt.paths(), &["/a/b/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if `path` is invalid.
    pub fn with_subdir_mut<'b, F, R>(&'b mut self, path: &[&'a str], f: F) -> Result<R>
    where
        F: FnOnce(&'b mut DTree<'a>) -> R,
    {
        if path.is_empty() {
            return Err(DirError::InvalidChild("empty path"));
        }

        let paths: Vec<&'a str> = path.iter().rev().cloned().collect();

        self.subdir_mut(paths, f)
    }

    fn subdir_mut<'b, F, R>(&'b mut self, mut path: Vec<&'a str>, f: F) -> Result<R>
    where
        F: FnOnce(&'b mut DTree<'a>) -> R,
    {
        if path.is_empty() {
            return Ok(f(self));
        }

        let name = path.pop().unwrap();

        for x in &mut self.children {
            if x.name == name {
                return x.subdir.subdir_mut(path, f);
            }
        }

        Err(DirError::InvalidChild(name))
    }

    /// Produce a list of the paths to each reachable leaf, in no particular order.  Path
    /// components are prefixed by `/`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::DTree;
    /// let mut dt = DTree::new();
    /// dt.mkdir("a").unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("b").unwrap()).unwrap();
    /// dt.with_subdir_mut(&["a"], |dt| dt.mkdir("c").unwrap()).unwrap();
    /// let mut paths = dt.paths();
    /// paths.sort();
    /// assert_eq!(&paths, &["/a/b/", "/a/c/"]);
    /// ```
    pub fn paths(&self) -> Vec<String> {
        let mut pathvec: Vec<String> = Vec::new();

        for x in &self.children {
            for y in x.paths() {
                pathvec.push("/".to_owned() + &y);
            }
        }
        pathvec
    }

    fn validdir(&self) {
    }
}

impl<'a> OsState<'a> {
    /// Create a new directory tree in the operating system.  Current working directory is the
    /// root.
    pub fn new() -> Self {
        Self::default()
    }

    /// If `path` is empty, change the working directory to the root.  Otherwise change the
    /// working directory to the subdirectory given by `path` relative to the current working
    /// directory.  (There is no notion of `.` or `..`: `path` must be a valid sequence of
    /// component names.)
    ///
    /// # Examples
    ///
    /// ```
    /// # use dtree::OsState;
    /// let mut s = OsState::new();
    /// s.mkdir("a").unwrap();
    /// s.chdir(&["a"]).unwrap();
    /// s.mkdir("b").unwrap();
    /// s.chdir(&["b"]).unwrap();
    /// s.mkdir("c").unwrap();
    /// s.chdir(&[]).unwrap();
    /// assert_eq!(&s.paths().unwrap(), &["/a/b/c/"]);
    /// ```
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if the new working directory is invalid. On error, the original
    /// working directory will be retained.
    pub fn chdir(&mut self, path: &[&'a str]) -> Result<()> {
        if path.is_empty() {
            self.cwd.clear();
        } else {
            match self.dtree.subdir(self.cwd.clone(), |dir| {
                dir.with_subdir(path, |dest| dest.validdir())
            }) {
                Ok(_) => self.cwd.extend(path.iter().cloned()),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    /// Make a new subdirectory with the given `name` in the working directory.
    ///
    /// # Errors
    ///
    /// * `DirError::SlashInName` if `name` contains `/`.
    /// * `DirError::InvalidChild` if the current working directory is invalid.
    /// * `DirError::DirExists` if `name` already exists.
    pub fn mkdir(&mut self, name: &'a str) -> Result<()> {
        if name.contains('/') {
            return Err(DirError::SlashInName(name));
        } else if self.cwd.is_empty() {
            return self.dtree.mkdir(name);
        }

        let mut pathvec = self.cwd.clone();
        pathvec.reverse();

        self.dtree
            .subdir_mut(pathvec, |dtree| dtree.mkdir(name).unwrap())
    }

    /// Produce a list of the paths from the working directory to each reachable leaf, in no
    /// particular order.  Path components are separated by `/`.
    ///
    /// # Errors
    ///
    /// * `DirError::InvalidChild` if the current working directory is invalid.
    pub fn paths(&self) -> Result<Vec<String>> {
        if self.cwd.is_empty() {
            return Ok(self.dtree.paths());
        }

        let mut pathvec = self.cwd.clone();
        pathvec.reverse();

        self.dtree.subdir(pathvec, |dir| dir.paths())
    }
}
