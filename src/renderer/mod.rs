use std::io::{Write, stdout};

use crossterm::{cursor::MoveToNextLine, queue, style::Print};

pub mod tui;

pub fn render_lines(lines: &[String]) -> Result<(), String> {
    let mut stdout = stdout();
    for line in lines {
        queue!(stdout, Print(line)).map_err(|err| err.to_string())?;
        queue!(stdout, MoveToNextLine(0)).map_err(|err| err.to_string())?;
    }
    stdout.flush().map_err(|err| err.to_string())?;
    Ok(())
}
