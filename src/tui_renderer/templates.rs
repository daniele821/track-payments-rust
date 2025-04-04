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

fn downscale_to_biggest_factor(values: &[u32], max_length: u32) -> Vec<u32> {
    let mut scaling_factor = values.len() / max_length as usize;
    while scaling_factor * (max_length as usize) < values.len() {
        scaling_factor += 1;
    }
    #[allow(clippy::cast_possible_truncation)]
    return values
        .chunks(scaling_factor)
        .map(|chunk| chunk.iter().sum::<u32>() / chunk.len() as u32)
        .collect::<Vec<u32>>();
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
    if values.is_empty() || max_width == 0 || max_height == 0 {
        return simple_rectangle(" ", max_width, max_height);
    }

    if values.len() > max_width as usize {
        let compacted_data = downscale_to_biggest_factor(values, max_width);
        return bar_graph_vertical(&compacted_data, max_width, max_height, cutout);
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
    #[allow(clippy::cast_possible_truncation)]
    DrawnArea::new(lines, actual_len as u32, max_height)
}

#[must_use]
pub fn bar_graph_horizontal(
    values: &[u32],
    max_width: u32,
    max_height: u32,
    cutout: u32,
) -> DrawnArea {
    if values.is_empty() || max_width == 0 || max_height == 0 {
        return simple_rectangle(" ", max_width, max_height);
    }

    if values.len() > max_height as usize {
        let compacted_data = downscale_to_biggest_factor(values, max_height);
        return bar_graph_vertical(&compacted_data, max_width, max_height, cutout);
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::downscale_to_biggest_factor;

    #[test]
    pub fn downscale_data() {
        let data = [1, 3, 5, 9, 10];
        assert_eq!(vec![2, 7, 10], downscale_to_biggest_factor(&data, 4));
    }
}
