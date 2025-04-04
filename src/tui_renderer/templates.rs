#![allow(unused, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

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
    values
        .chunks(scaling_factor)
        .map(|chunk| chunk.iter().sum::<u32>() / chunk.len() as u32)
        .collect::<Vec<u32>>()
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
    let actual_width = usize::max(len, max_width as usize / len * len);
    let factor = actual_width / len;
    let unit_heigh = f64::from(max_height) / f64::from(max);
    let cached_spaces = " ".repeat(factor);

    for i in (1..=max_height).rev() {
        let mut str = String::with_capacity(actual_width);
        for &j in values {
            if f64::from(i - 1) < f64::from(j) * unit_heigh - 0.5 {
                let mut color = Color::DarkGreen;
                if j >= cutout {
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
    DrawnArea::new(lines, actual_width as u32, max_height)
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
        return bar_graph_horizontal(&compacted_data, max_width, max_height, cutout);
    }

    let mut lines = Vec::with_capacity(max_height as usize);
    let max = u32::max(1, *values.iter().max().unwrap_or(&0));
    let len = values.len();
    let actual_height = max_height as usize / len * len;
    let factor = actual_height / len;
    let unit_width = f64::from(max_width) / f64::from(max);

    for &val in values {
        let mut color = Color::Green;
        if val >= cutout {
            color = Color::Red;
        }
        let bar_len = (f64::from(val) * unit_width).trunc() as usize;
        let rem_len = max_width as usize - bar_len;
        let str = format!("{}{}", " ".repeat(bar_len).on(color), " ".repeat(rem_len));
        for i in 0..factor {
            lines.push(str.clone());
        }
    }

    DrawnArea::new(lines, max_width, actual_height as u32)
}

#[must_use]
pub fn bar_graph_horizontal_label(
    values: &[u32],
    max_width: u32,
    max_height: u32,
    cutout: u32,
) -> DrawnArea {
    const MIN_GRAPH_SIZE: usize = 5;
    if (max_height as usize) < values.len() {
        return bar_graph_horizontal(values, max_width, max_height, cutout);
    }

    let max_index_len = values.len().to_string().len();
    let left_len = max_index_len + 2;

    let max_value = format!("{:03}", values.iter().max().unwrap_or(&0));
    let len = max_value.len();
    let max_value = format!("{}.{}", &max_value[..len - 2], &max_value[len - 2..]);
    let max_value_len = max_value.len() + 1;
    let right_len = max_value_len + 2;

    let label_len = left_len + right_len;

    if (max_width as usize) < label_len + MIN_GRAPH_SIZE {
        return bar_graph_horizontal(values, max_width, max_height, cutout);
    }

    let factor = (max_height as usize) / values.len();
    let cached_left = " ".repeat(left_len);
    let cached_right = " ".repeat(right_len);

    let actual_max_width = max_width as usize - label_len;
    let mut graph = bar_graph_horizontal(values, actual_max_width as u32, max_height, cutout);
    for (index, value) in values.iter().enumerate() {
        let index_fmt = format!(" {:>2} ", index + 1);
        let len = max_value.len();
        let tmp_fmt = format!("{:>len$}", format!("{:03}", values[index]));
        let tmp = tmp_fmt.len() - 2;
        let value_fmt = format!("{}.{}\u{20ac} ", &tmp_fmt[..tmp], &tmp_fmt[tmp..]);
        if let Some(line) = graph.area.get_mut(index * factor) {
            *line = format!("{index_fmt}{line}{value_fmt}");
        }
        for i in 1..factor {
            if let Some(line) = graph.area.get_mut(index * factor + i) {
                *line = format!("{cached_left}{line}{cached_right}");
            }
        }
    }
    graph.width += label_len as u32;

    graph
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
