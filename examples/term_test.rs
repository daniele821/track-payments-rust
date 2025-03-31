use std::{thread::sleep, time::Duration};

use crossterm::terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen).unwrap();
    sleep(Duration::from_secs(10));
    println!("ciao");
    disable_raw_mode().unwrap();
}
