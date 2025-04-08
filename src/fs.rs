use std::{
    env::current_exe,
    path::{Path, PathBuf},
};

fn get_dir(path: &Path) -> Option<PathBuf> {
    path.parent().map(|dir| dir.to_path_buf())
}

pub fn get_exe_path() -> PathBuf {
    current_exe().expect("failed to retrieve current executable")
}

pub fn get_exe_dir() -> PathBuf {
    get_dir(&get_exe_path()).expect("failed to retrieve directory of current executable")
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
