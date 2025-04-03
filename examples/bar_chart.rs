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
    io::{self, Write, stdout},
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
        Hide,
        crossterm::cursor::MoveTo(0, 0),
    )?;

    render();

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                Event::Resize(_, _) => render(),
                _ => {}
            }
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
    let width = crossterm::terminal::size().unwrap().0 - 2;
    let height = crossterm::terminal::size().unwrap().1 - 2;
    let graph = track_payments_rust::tui_renderer::templates::bar_graph_vertical(
        &[
            0, 752, 707, 2787, 1019, 864, 890, 2853, 0, 0, 841, 989, 678, 990, 1812, 0, 733, 714,
            782, 931, 1722, 1803, 862, 1278, 1079, 857, 558, 1450, 536, 857, 649,
        ],
        u32::from(width),
        u32::from(height),
        1000,
    );
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    for _ in 0..width + 2 {
        print!("-");
    }
    println!("\r");
    (0..graph.len()).for_each(|line| {
        print!("|{}|\n\r", graph[line]);
    });
    for _ in 0..width + 2 {
        print!("-");
    }
    std::io::stdout().flush().unwrap();
}
