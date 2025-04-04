#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, deprecated)]

use atty::Stream;
use chrono::{Datelike, TimeZone, Utc};
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
    io::{self, Read, Write, stdout},
    time::Duration,
};
use track_payments_rust::payments::{AllPaymentsJsonLegacy, PaymentId};

fn main() -> io::Result<()> {
    let mut data = vec![
        0, 752, 707, 2787, 1019, 864, 890, 2853, 0, 0, 841, 989, 678, 990, 1812, 0, 733, 714, 782,
        931, 1722, 1803, 862, 1278, 1079, 857, 558, 1450, 536, 857, 649,
    ];
    let mut ignore = vec![];

    if atty::isnt(Stream::Stdin) {
        let now = Utc::now();
        let start_of_month = Utc.ymd(now.year(), now.month(), 1).and_hms(0, 0, 0);
        let (next_year, next_month) = if now.month() == 12 {
            (now.year() + 1, 1)
        } else {
            (now.year(), now.month() + 1)
        };
        let start_of_next_month = Utc.ymd(next_year, next_month, 1).and_hms(0, 0, 0);
        let duration = start_of_next_month.signed_duration_since(start_of_month);
        let days_in_month = duration.num_days();

        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let all_payments = AllPaymentsJsonLegacy::from_json(&input).unwrap();
        let all_payments = all_payments.to_api().unwrap();

        let range = all_payments.payments().range(
            &PaymentId::new(start_of_month.into())..&PaymentId::new(start_of_next_month.into()),
        );
        data = vec![0; days_in_month as usize];
        ignore = (now.day()..days_in_month as u32).collect::<Vec<u32>>();
        for (id, _det) in range {
            let orders = all_payments.payments().get(id).unwrap().orders();
            let mut sum = 0;
            for (orderid, orderdet) in orders {
                sum += orderid.unit_price() * orderdet.quantity();
            }
            data[id.date().get_fields().unwrap().day0() as usize] += sum;
        }
    }

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

    render(&data, &ignore);

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
                    render(&data, &ignore);
                    (x, y) = (new_x, new_y);
                }
                _ => {}
            }
        }
        if crossterm::terminal::size().unwrap() != (x, y) {
            render(&data, &ignore);
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

fn render(data: &[u32], ignore: &[u32]) {
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
    let graph = track_payments_rust::tui::bar_graph_horizontal_label(
        data,
        u32::from(width),
        u32::from(height),
        1000,
        ignore,
        true,
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
