use std::fs;
use std::path::Path;

pub trait PathExt {
    fn is_dir_empty(&self) -> bool;
    fn clear_dir(&self) -> std::io::Result<()>;
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
}
