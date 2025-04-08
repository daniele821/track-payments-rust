use std::{
    env::current_exe,
    path::{Path, PathBuf},
};

fn get_dir(path: &Path) -> Result<PathBuf, String> {
    path.parent()
        .map(|dir| dir.to_path_buf())
        .ok_or_else(|| format!("unable to get parent of {}", path.to_string_lossy()))
}

pub fn get_exe_path() -> Result<PathBuf, String> {
    current_exe().map_err(|err| err.to_string())
}

pub fn get_exe_dir() -> Result<PathBuf, String> {
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
