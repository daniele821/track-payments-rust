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
    // TODO: when space is smaller then data, then compact it:
    //      - by integer factors: /2, /3, /4, ...
    // IDEA: when space is not a multiple of the data len, WHAT TO DO?
    //      - maybe return a tuple with the actual drawn height and width?
    //      - rename width/height as new_width/new_height
    if values.is_empty() {
        return simple_rectangle(" ", width, height);
    }

    let mut lines = Vec::with_capacity(height as usize);
    let max = u32::max(1, *values.iter().max().unwrap_or(&0));
    let len = values.len();
    let actual_len = usize::max(len, width as usize / len * len);
    let rem = usize::saturating_sub(width as usize, actual_len);
    let rem_str = " ".repeat(rem);
    let factor = actual_len / len;
    let unit_heigh = f64::from(height) / f64::from(max);
    let cached_spaces = " ".repeat(factor);

    for i in (1..=height).rev() {
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
        str.push_str(&rem_str);
        lines.push(str);
    }
    lines
}
