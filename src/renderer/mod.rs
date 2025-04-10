pub mod tui;
pub mod tui_;

use crate::error::{Error, Result};
use crossterm::{cursor::MoveToNextLine, queue, style::Print};
use std::io::{Write, stdout};

pub fn render_lines(lines: &[String]) -> Result<()> {
    let mut stdout = stdout();
    for line in lines {
        queue!(stdout, Print(line)).map_err(Error::from_generic)?;
        queue!(stdout, MoveToNextLine(0)).map_err(Error::from_generic)?;
    }
    stdout.flush().map_err(Error::from_generic)?;
    Ok(())
}
