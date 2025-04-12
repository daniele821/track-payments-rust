use crate::error::{Error, Result};
use std::{
    env::current_exe,
    fs::read,
    fs::write,
    path::{Path, PathBuf},
};

fn get_dir(path: &Path) -> Option<PathBuf> {
    path.parent().map(|dir| dir.to_path_buf())
}

pub fn get_exe_path() -> Result<PathBuf> {
    current_exe().map_err(Error::from_generic)
}

pub fn get_exe_dir() -> Result<PathBuf> {
    get_dir(&get_exe_path()?)
        .ok_or_else(|| String::from("failed to get executable directory"))
        .map_err(Error::from_generic)
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    read(path).map_err(Error::FileError)
}

pub fn write_file<P: AsRef<Path>>(path: P, contents: Vec<u8>) -> Result<()> {
    write(path, contents).map_err(Error::FileError)
}

#[cfg(test)]
mod tests {
    use super::{get_exe_dir, get_exe_path};

    #[test]
    fn get_exe() {
        let exe_path = get_exe_path();
        let exe_dir = get_exe_dir();
        println!("{exe_path:?}");
        println!("{exe_dir:?}");
    }
}
