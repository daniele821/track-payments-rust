use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, KeyCode, KeyModifiers},
    terminal::{EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    let start = Instant::now();
    crossterm::execute!(stdout, EnterAlternateScreen).unwrap();
    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                println!("Key pressed: {:?}\r", key_event.code);
                if key_event.code == KeyCode::Esc {
                    break;
                }
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    println!("Ctrl+C detected. Exiting...");
                    break;
                }
            }
        }
        println!("{}\r", start.elapsed().as_millis());
    }
    disable_raw_mode().unwrap();
}
