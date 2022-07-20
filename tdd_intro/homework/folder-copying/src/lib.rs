use std::{
    io,
    path::{Path, PathBuf},
};

pub trait FileSystem {
    fn copy_file(&mut self, from: &Path, to: &Path) -> io::Result<()>;
    fn create_folder(&mut self, path: &Path) -> io::Result<()>;
    fn is_dir(&mut self, path: &Path) -> bool;
    fn list_folder(&mut self, path: &Path) -> io::Result<Vec<PathBuf>>;
}

pub struct FolderCopier<'a> {
    file_system: &'a mut dyn FileSystem,
}

impl<'a> FolderCopier<'a> {
    pub fn new(file_system: &'a mut dyn FileSystem) -> Self {
        Self { file_system }
    }

    pub fn copy_folder<P: AsRef<Path>, Q: AsRef<Path>>(
        &mut self,
        from: P,
        to: Q,
    ) -> io::Result<()> {
        let from = from.as_ref();
        let to = to.as_ref();

        if from == to {
            return Err(io::Error::new(io::ErrorKind::Other, "dest folder is a source folder"));
        }

        let items = self.file_system.list_folder(from)?;

        if items.is_empty() {
            return Ok(());
        }

        self.file_system.create_folder(to)?;

        println!("copy folder: {:?} {:?}", from, to);

        for item in items {
            println!("from, item: {:?} {:?}", from, item);

            let mut to = to.to_owned();
            to.push(item.clone());

            let mut from = from.to_owned();
            from.push(item.clone());

            if self.file_system.is_dir(&from) {
                self.copy_folder(&from, to)?;
            } else {
                self.file_system.copy_file(&from, &to)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        io,
        path::{Path, PathBuf},
    };

    use crate::{FileSystem, FolderCopier};

    #[derive(Debug)]
    enum Item {
        File,
        Directory(Vec<PathBuf>),
    }

    // very dumb file system mock
    #[derive(Debug)]
    struct FileSystemMock {
        // holds test file system hierarchy
        pub items: HashMap<PathBuf, Item>,
        // holds paths that our class copied
        pub copied: HashMap<PathBuf, Item>,
    }

    impl FileSystemMock {
        pub fn new(items: HashMap<PathBuf, Item>) -> Self {
            Self {
                items,
                copied: HashMap::new(),
            }
        }
    }

    impl FileSystem for FileSystemMock {
        fn copy_file(&mut self, from: &Path, to: &Path) -> io::Result<()> {
            match self.items.get(from) {
                Some(Item::File) => {
                    self.copied.insert(to.to_owned(), Item::File);
                    Ok(())
                }
                Some(Item::Directory(_)) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Provided path is a folder instead of the file",
                )),
                None => Err(io::Error::new(io::ErrorKind::Other, "File not found")),
            }
        }

        fn create_folder(&mut self, path: &Path) -> io::Result<()> {
            if self.copied.get(path).is_some() {
                return Err(io::Error::new(io::ErrorKind::Other, "Already exist"));
            }

            self.copied
                .insert(path.to_owned(), Item::Directory(Vec::new()));

            Ok(())
        }

        fn is_dir(&mut self, path: &Path) -> bool {
            match self.items.get(path).unwrap() {
                Item::Directory(_) => true,
                _ => false,
            }
        }

        fn list_folder(&mut self, path: &Path) -> io::Result<Vec<PathBuf>> {
            match self
                .items
                .get(path)
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "folder not found"))?
            {
                Item::File => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "provided path is a file instead of folder",
                )),
                Item::Directory(items) => Ok(items.clone()),
            }
        }
    }

    #[test]
    fn copy_file() {
        let mut items = HashMap::new();

        items.insert(
            Path::new("from_dir").to_owned(),
            Item::File,
        );

        let mut file_system = FileSystemMock::new(items);

        let mut folder_copier = FolderCopier::new(&mut file_system);

        assert!(folder_copier.copy_folder("from_dir", "from_dir").is_err());
    }

    #[test]
    fn same_folder() {
        let mut items = HashMap::new();

        items.insert(
            Path::new("from_dir").to_owned(),
            Item::Directory(Vec::new()),
        );

        let mut file_system = FileSystemMock::new(items);

        let mut folder_copier = FolderCopier::new(&mut file_system);

        assert!(folder_copier.copy_folder("from_dir", "from_dir").is_err());
    }

    #[test]
    fn empty_folder() {
        let mut items = HashMap::new();

        items.insert(
            Path::new("from_dir").to_owned(),
            Item::Directory(Vec::new()),
        );

        let mut file_system = FileSystemMock::new(items);

        let mut folder_copier = FolderCopier::new(&mut file_system);

        folder_copier.copy_folder("from_dir", "to_dir").unwrap();

        assert!(file_system
            .copied
            .is_empty());
    }

    #[test]
    fn folder_with_nested_folders() {
        let mut items = HashMap::new();

        items.insert(
            Path::new("from_dir").to_owned(),
            Item::Directory(vec![
                Path::new("foo.txt").to_owned(),
                Path::new("bar.txt").to_owned(),
                Path::new("nested1").to_owned(),
            ]),
        );
        items.insert(Path::new("from_dir\\foo.txt").to_owned(), Item::File);
        items.insert(Path::new("from_dir\\bar.txt").to_owned(), Item::File);
        items.insert(
            Path::new("from_dir\\nested1").to_owned(),
            Item::Directory(vec![
                Path::new("nn1.txt").to_owned(),
                Path::new("nn2.txt").to_owned(),
                Path::new("nested2").to_owned(),
            ]),
        );
        items.insert(Path::new("from_dir\\nested1\\nn1.txt").to_owned(), Item::File);
        items.insert(Path::new("from_dir\\nested1\\nn2.txt").to_owned(), Item::File);
        items.insert(
            Path::new("from_dir\\nested1\\nested2").to_owned(),
            Item::Directory(vec![
                Path::new("nn21.txt").to_owned(),
                Path::new("nn22.txt").to_owned(),
            ]),
        );
        items.insert(Path::new("from_dir\\nested1\\nested2\\nn21.txt").to_owned(), Item::File);
        items.insert(Path::new("from_dir\\nested1\\nested2\\nn22.txt").to_owned(), Item::File);

        let mut file_system = FileSystemMock::new(items);

        let mut folder_copier = FolderCopier::new(&mut file_system);

        folder_copier.copy_folder("from_dir", "to_dir").unwrap();

        println!("===============");
        println!("{:?}", file_system.copied);
        println!("===============");

        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\foo.txt").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\bar.txt").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\nested1").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\nested1\\nn1.txt").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\nested1\\nn2.txt").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\nested1\\nested2").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\nested1\\nested2\\nn21.txt").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\nested1\\nested2\\nn22.txt").to_owned()));
    }

    #[test]
    fn folder_with_few_files() {
        let mut items = HashMap::new();

        items.insert(
            Path::new("from_dir").to_owned(),
            Item::Directory(vec![
                Path::new("foo.txt").to_owned(),
                Path::new("bar.txt").to_owned(),
                Path::new("ano.txt").to_owned(),
            ]),
        );
        items.insert(Path::new("from_dir\\foo.txt").to_owned(), Item::File);
        items.insert(Path::new("from_dir\\bar.txt").to_owned(), Item::File);
        items.insert(Path::new("from_dir\\ano.txt").to_owned(), Item::File);

        let mut file_system = FileSystemMock::new(items);

        let mut folder_copier = FolderCopier::new(&mut file_system);

        folder_copier.copy_folder("from_dir", "to_dir").unwrap();

        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\foo.txt").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\bar.txt").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\ano.txt").to_owned()));
    }

    #[test]
    fn folder_with_one_file() {
        let mut items = HashMap::new();

        items.insert(
            Path::new("from_dir").to_owned(),
            Item::Directory(vec![Path::new("foo.txt").to_owned()]),
        );
        items.insert(Path::new("from_dir\\foo.txt").to_owned(), Item::File);

        let mut file_system = FileSystemMock::new(items);

        let mut folder_copier = FolderCopier::new(&mut file_system);

        folder_copier.copy_folder("from_dir", "to_dir").unwrap();

        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir").to_owned()));
        assert!(file_system
            .copied
            .contains_key(&Path::new("to_dir\\foo.txt").to_owned()));
    }
}
