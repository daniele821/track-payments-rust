use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    // Enable mouse capture
    execute!(
        stdout(),
        EnableMouseCapture,
        EnterAlternateScreen,
        DisableLineWrap,
        // Hide,
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
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    let mut index = 0;
    (0..graph.len()).for_each(|line| {
        index += 1;
        if index < graph.len() {
            print!("|{}|\n\r", graph[line]);
        } else {
            print!("|{}|\r", graph[line]);
        }
    });
}
