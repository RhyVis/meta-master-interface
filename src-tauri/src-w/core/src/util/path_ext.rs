use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub trait PathExt {
    fn is_dir_empty(&self) -> bool;
    fn clear_dir(&self) -> std::io::Result<()>;
    fn calculate_size(&self) -> std::io::Result<u64>;
}

impl PathExt for Path {
    fn is_dir_empty(&self) -> bool {
        if !self.is_dir() {
            return false;
        }
        match fs::read_dir(self) {
            Ok(mut entries) => entries.next().is_none(),
            Err(_) => false,
        }
    }

    fn clear_dir(&self) -> std::io::Result<()> {
        let read_dir = fs::read_dir(self)?;
        for entry in read_dir {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                fs::remove_file(entry.path())?;
            } else if entry.file_type()?.is_dir() {
                fs::remove_dir_all(entry.path())?;
            }
        }
        Ok(())
    }

    fn calculate_size(&self) -> std::io::Result<u64> {
        if !self.exists() {
            Ok(0)
        } else if self.is_file() {
            Ok(self.metadata()?.len())
        } else if self.is_dir() {
            let walker = WalkDir::new(&self);
            let mut total_size = 0;

            for entry in walker.into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    total_size += entry.metadata()?.len();
                }
            }

            Ok(total_size)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path is neither a file nor a directory",
            ))
        }
    }
}
