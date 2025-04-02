use crossterm::style::{Color, Stylize};

#[must_use]
pub fn bar_graph_vertical(values: &[u32], width: u32, height: u32, cutout: u32) -> Vec<String> {
    // if values.len() == 0, returns empty strings of correct len
    // assert result is ALWAYS of size equal to width and height passed as input
    let max = *values.iter().max().unwrap_or(&0);
    let len = values.len();
    let actual_len = usize::max(len, width as usize / len * len);
    let factor = actual_len / len;
    let unit_heigh = f64::from(height) / f64::from(max);
    let mut lines = Vec::with_capacity(height as usize);
    println!("{len},{actual_len},{factor}");
    for i in (1..=height).rev() {
        let mut str = String::with_capacity(actual_len);
        for j in values {
            if f64::from(i - 1) < f64::from(*j) * unit_heigh - 0.5 {
                let mut color = Color::DarkGreen;
                if *j >= cutout {
                    color = Color::DarkRed;
                }
                let tmp_str = format!("▏{}", " ".repeat(factor - 1))
                    .on(color)
                    .with(Color::Black)
                    .to_string();
                str.push_str(&tmp_str);
            } else {
                str.push_str(&" ".repeat(factor));
            }
        }
        lines.push(str);
    }
    lines
}
