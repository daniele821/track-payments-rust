use crossterm::style::{Color, Stylize};

#[must_use]
pub fn simple_rectangle(elem: &str, width: u32, height: u32) -> Vec<String> {
    let mut lines = Vec::with_capacity(height as usize);
    let empty_line = elem.repeat(width as usize);
    for _ in 0..height {
        lines.push(empty_line.clone());
    }
    lines
}

#[must_use]
pub fn bar_graph_vertical(values: &[u32], width: u32, height: u32, cutout: u32) -> Vec<String> {
    if values.is_empty() {
        return simple_rectangle(" ", width, height);
    }

    let mut lines = Vec::with_capacity(height as usize);
    let max = u32::max(1, *values.iter().max().unwrap_or(&0));
    let len = values.len();
    let actual_len = usize::max(len, width as usize / len * len);
    let rem = usize::saturating_sub(width as usize, actual_len);
    let rem_left = " ".repeat(rem / 2);
    let rem_right = " ".repeat(rem / 2 + rem % 2);
    let factor = actual_len / len;
    let unit_heigh = f64::from(height) / f64::from(max);
    let mut prev;
    let cached_spaces = " ".repeat(factor - 1);

    for i in (1..=height).rev() {
        prev = false;
        let mut str = String::with_capacity(actual_len);
        str.push_str(&rem_left);
        for j in values {
            if f64::from(i - 1) < f64::from(*j) * unit_heigh - 0.5 {
                let mut color = Color::DarkGreen;
                if *j >= cutout {
                    color = Color::DarkRed;
                }
                let tmp_str = format!("▏{cached_spaces}")
                    .on(color)
                    .with(Color::Black)
                    .to_string();
                str.push_str(&tmp_str);
                prev = true;
            } else if i == 1 {
                str.push_str(&"▁".repeat(factor).with(Color::DarkGreen).to_string());
            } else {
                let mut sep = "▏";
                if !prev {
                    sep = " ";
                }
                let tmp_str = format!("{sep}{cached_spaces}")
                    .with(Color::Black)
                    .to_string();
                str.push_str(&tmp_str);
                prev = false;
            }
        }
        str.push_str(&rem_right);
        lines.push(str);
    }
    lines
}
