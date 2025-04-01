use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{
        DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    // Enable raw mode for direct event handling
    enable_raw_mode()?;

    // Enable mouse capture
    execute!(
        stdout(),
        EnableMouseCapture,
        EnterAlternateScreen,
        DisableLineWrap
    )?;

    println!("Move your mouse or click in the terminal. Press 'q' to quit.");

    loop {
        match event::read()? {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
            Event::Mouse(mouse_event) => {
                handle_mouse_event(mouse_event);
            }
            _ => {} // Ignore other events
        }
    }

    // Cleanup (this code won't actually run in this example due to the infinite loop)
    disable_raw_mode()?;
    execute!(
        stdout(),
        DisableMouseCapture,
        LeaveAlternateScreen,
        EnableLineWrap,
    )?;
    Ok(())
}

fn handle_mouse_event(mouse_event: MouseEvent) {
    match mouse_event.kind {
        MouseEventKind::Down(button) => {
            println!(
                "Mouse button {:?} pressed at {:?}\r",
                button,
                (mouse_event.column, mouse_event.row)
            );
        }
        MouseEventKind::Up(button) => {
            println!(
                "Mouse button {:?} released at {:?}\r",
                button,
                (mouse_event.column, mouse_event.row)
            );
        }
        MouseEventKind::Drag(button) => {
            println!(
                "Mouse dragged with {:?} to {:?}\r",
                button,
                (mouse_event.column, mouse_event.row)
            );
        }
        MouseEventKind::Moved => {
            println!(
                "Mouse moved to {:?}\r",
                (mouse_event.column, mouse_event.row)
            );
        }
        MouseEventKind::ScrollUp => {
            println!(
                "Mouse scrolled up at {:?}\r",
                (mouse_event.column, mouse_event.row)
            );
        }
        MouseEventKind::ScrollDown => {
            println!(
                "Mouse scrolled down at {:?}\r",
                (mouse_event.column, mouse_event.row)
            );
        }
        _ => {}
    }
}
