use std::{
    io,
    path::{Path, PathBuf},
};

pub trait FileSystem {
    fn copy_file(&self, from: &Path, to: &Path) -> io::Result<()>;
    fn create_folder(&self, path: &Path) -> io::Result<()>;
    fn is_file(&self, path: &Path) -> bool;
    fn list_folder(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
}

pub struct FolderCopier {
    file_system: Box<dyn FileSystem>,
}

impl FolderCopier {
    pub fn new(file_system: Box<dyn FileSystem>) -> Self {
        Self { file_system }
    }

    pub fn copy_folder<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        io,
        path::{Path, PathBuf},
    };

    use crate::FileSystem;

    struct FileSystemMock<'a> {
        pub items: HashMap<&'a Path, bool>,
        pub copied: HashMap<&'a Path, bool>,
    }

    impl<'a> FileSystem for FileSystemMock<'a> {
        fn copy_file(&self, from: &Path, to: &Path) -> io::Result<()> {
            todo!()
        }

        fn create_folder(&self, path: &Path) -> io::Result<()> {
            todo!()
        }

        fn is_file(&self, path: &Path) -> bool {
            *self.items.get(path).unwrap()
        }

        fn list_folder(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
            todo!()
        }
    }

    #[test]
    fn folder_with_one_file() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
