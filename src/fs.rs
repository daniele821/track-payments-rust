use std::{env::current_exe, path::PathBuf};

pub fn get_exe_path() -> Result<PathBuf, String> {
    current_exe().map_err(|err| err.to_string())
}
