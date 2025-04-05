use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::{Color, Stylize},
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

    let (mut x, mut y) = (0, 0);
    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                Event::Resize(new_x, new_y) => {
                    render();
                    (x, y) = (new_x, new_y);
                }
                _ => {}
            }
        }
        if crossterm::terminal::size().unwrap() != (x, y) {
            render();
            (x, y) = crossterm::terminal::size().unwrap();
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
    let white = &" ".on(Color::White).to_string();
    let white2 = &"  ".on(Color::White).to_string();
    let symbols = [
        ["━", "┃", "┏", "┓", "┗", "┛"],
        ["─", "│", "┌", "┐", "└", "┘"],
        ["═", "║", "╔", "╗", "╚", "╝"],
        [white, white2, white2, white2, white2, white2],
    ];
    let box_sym = symbols[3];
    let width = crossterm::terminal::size().unwrap().0 - 4;
    let height = crossterm::terminal::size().unwrap().1 - 2;
    let graph = track_payments_rust::tui_renderer::templates::bar_graph_horizontal_label(
        &[
            0, 752, 707, 2787, 1019, 864, 890, 2853, 0, 0, 841, 989, 678, 990, 1812, 0, 733, 714,
            782, 931, 1722, 1803, 862, 1278, 1079, 857, 558, 1450, 536, 857, 649,
        ],
        u32::from(width),
        u32::from(height),
        1000,
        &[1, 2, 3, 4],
        false,
    );
    let width = graph.width;
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    print!("{}", box_sym[2]);
    for _ in 0..width {
        print!("{}", box_sym[0]);
    }
    print!("{}\n\r", box_sym[3]);
    (0..graph.area.len()).for_each(|line| {
        print!("{}{}{}\n\r", box_sym[1], graph.area[line], box_sym[1]);
    });
    print!("{}", box_sym[4]);
    for _ in 0..width {
        print!("{}", box_sym[0]);
    }
    print!("{}", box_sym[5]);
    std::io::stdout().flush().unwrap();
}
