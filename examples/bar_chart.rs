use crossterm::{
    cursor::{Hide, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{
        DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};
use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

fn main() -> io::Result<()> {
    // Enable raw mode for direct event handling
    enable_raw_mode()?;

    // Enable mouse capture
    execute!(
        stdout(),
        EnableMouseCapture,
        EnterAlternateScreen,
        DisableLineWrap,
        Hide,
        crossterm::cursor::MoveTo(0, 0),
    )?;

    loop {
        sleep(Duration::from_millis(30));
        render();
        match event::read()? {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
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
        Show,
    )?;
    Ok(())
}

fn render() {
    let width = crossterm::terminal::size().unwrap().0;
    let width = u32::from(width - 2);
    let height = crossterm::terminal::size().unwrap().1 as u32;
    let graph = track_payments_rust::tui_renderer::templates::bar_graph_vertical(
        &[
            0, 752, 707, 2787, 1019, 864, 890, 2853, 0, 0, 841, 989, 678, 990, 1812, 0, 733, 714,
            782, 931, 1722, 1803, 862, 1278, 1079, 857, 558, 1450, 536, 857, 649,
        ],
        width,
        height,
        1000,
    );
    for line in &graph {
        print!("|{line}|\n\r");
    }
}
