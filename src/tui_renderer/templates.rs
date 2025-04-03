#![allow(unused, clippy::missing_panics_doc)]

use crossterm::style::{Color, Stylize};

#[derive(Debug)]
pub struct DrawnArea {
    pub area: Vec<String>,
    pub width: u32,
    pub height: u32,
}

impl DrawnArea {
    #[must_use]
    pub fn new(area: Vec<String>, width: u32, height: u32) -> Self {
        Self {
            area,
            width,
            height,
        }
    }
}

#[must_use]
pub fn simple_rectangle(elem: &str, width: u32, height: u32) -> DrawnArea {
    let mut lines = Vec::with_capacity(height as usize);
    let empty_line = elem.repeat(width as usize);
    for _ in 0..height {
        lines.push(empty_line.clone());
    }
    DrawnArea::new(lines, width, height)
}

#[must_use]
pub fn bar_graph_vertical(
    values: &[u32],
    max_width: u32,
    max_height: u32,
    cutout: u32,
) -> DrawnArea {
    // TODO: when space is smaller then data, then compact it:
    //      - by integer factors: /2, /3, /4, ...

    if values.is_empty() || max_width == 0 || max_height == 0 {
        return simple_rectangle(" ", max_width, max_height);
    }

    let mut lines = Vec::with_capacity(max_height as usize);
    let max = u32::max(1, *values.iter().max().unwrap_or(&0));
    let len = values.len();
    let actual_len = usize::max(len, max_width as usize / len * len);
    let factor = actual_len / len;
    let unit_heigh = f64::from(max_height) / f64::from(max);
    let cached_spaces = " ".repeat(factor);

    for i in (1..=max_height).rev() {
        let mut str = String::with_capacity(actual_len);
        for j in values {
            if f64::from(i - 1) < f64::from(*j) * unit_heigh - 0.5 {
                let mut color = Color::DarkGreen;
                if *j >= cutout {
                    color = Color::DarkRed;
                }
                let tmp_str = cached_spaces.to_string().on(color).to_string();
                str.push_str(&tmp_str);
            } else {
                str.push_str(&cached_spaces);
            }
        }
        lines.push(str);
    }
    DrawnArea::new(lines, u32::try_from(actual_len).unwrap(), max_height)
}
