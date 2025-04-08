use std::{
    env::current_exe,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};

fn get_dir(path: &Path) -> Result<PathBuf> {
    path.parent()
        .map(|dir| dir.to_path_buf())
        .ok_or_else(|| format!("unable to get parent of {}", path.to_string_lossy()))
        .map_err(Error::from_generic)
}

pub fn get_exe_path() -> Result<PathBuf> {
    current_exe().map_err(Error::from_generic)
}

pub fn get_exe_dir() -> Result<PathBuf> {
    get_dir(&get_exe_path()?)
}

#[cfg(test)]
mod tests {
    use super::{get_exe_dir, get_exe_path};

    #[test]
    fn get_exe() {
        let exe_path = get_exe_path().unwrap();
        let exe_dir = get_exe_dir().unwrap();
        println!("{exe_path:?}");
        println!("{exe_dir:?}");
    }
}
