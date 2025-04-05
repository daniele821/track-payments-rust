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

const COLOR_BORDER: Color = Color::White;
const COLOR_TEXT: Color = Color::White;
const COLOR_GOOD: Color = Color::DarkGreen;
const COLOR_BAD: Color = Color::DarkRed;
const COLOR_EMPTY: Color = Color::DarkGrey;
const COLOR_CUTOUT: Color = Color::Yellow;

// https://en.wikipedia.org/wiki/Box-drawing_characters
const STR_EMPTY: &str = " ";
const STR_CUTOUT_VERT: &str = "╏";
const STR_CUTOUT_HORIZ: &str = "╍";

fn downscale_to_biggest_factor(
    values: &[u32],
    ignored: &[u32],
    max_length: u32,
) -> (Vec<u32>, Vec<u32>) {
    let max_length = max_length as usize;
    let mut scaling_factor = values.len() / max_length;
    while scaling_factor * max_length < values.len() {
        scaling_factor += 1;
    }

    let final_len = values.len().div_ceil(scaling_factor);
    let mut compacted_values = Vec::with_capacity(final_len);
    let mut compacted_ignored = Vec::with_capacity(final_len);

    let mut buffer = Vec::<u32>::with_capacity(scaling_factor);

    for i in 0..final_len {
        for j in 0..scaling_factor {
            let index = scaling_factor * i + j;
            if !ignored.contains(&(index as u32)) {
                if let Some(&elem) = values.get(index) {
                    buffer.push(elem);
                }
            }
        }

        if buffer.is_empty() {
            compacted_ignored.push(i as u32);
            compacted_values.push(0);
        } else {
            compacted_values.push(buffer.iter().sum::<u32>() / buffer.len() as u32);
        }
        buffer.clear();
    }

    (compacted_values, compacted_ignored)
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
    ignored: &[u32],
) -> DrawnArea {
    if values.is_empty() || max_width == 0 || max_height == 0 {
        return simple_rectangle(STR_EMPTY, max_width, max_height);
    }

    if values.len() > max_width as usize {
        let (data, ignored) = downscale_to_biggest_factor(values, ignored, max_width);
        return bar_graph_vertical(&data, max_width, max_height, cutout, &ignored);
    }

    let mut lines = Vec::with_capacity(max_height as usize);
    let max = u32::max(1, *values.iter().max().unwrap_or(&0));
    let len = values.len();
    let actual_width = usize::max(len, max_width as usize / len * len);
    let factor = actual_width / len;
    let unit_heigh = f64::from(max_height) / f64::from(max);
    let cached_spaces = STR_EMPTY.repeat(factor);
    let cached_cutout = STR_CUTOUT_HORIZ
        .repeat(factor)
        .with(COLOR_CUTOUT)
        .to_string();

    let mut is_cutout_line = false;

    for i in (1..=max_height).rev() {
        let mut str = String::with_capacity(actual_width);
        for (index, &j) in values.iter().enumerate() {
            let mut cached = &cached_spaces;
            is_cutout_line = false;
            if f64::from(i - 1) >= f64::from(cutout) * unit_heigh
                && f64::from(i - 2) < f64::from(cutout) * unit_heigh
            {
                cached = &cached_cutout;
                is_cutout_line = true;
            }
            if ignored.contains(&(index as u32)) {
                str.push_str(&cached_spaces.to_string().on(COLOR_EMPTY).to_string());
            } else if f64::from(i - 1) < f64::from(j) * unit_heigh {
                let mut color = COLOR_GOOD;
                if j >= cutout {
                    color = COLOR_BAD;
                }
                let tmp_str = cached.to_string().on(color).to_string();
                str.push_str(&tmp_str);
            } else if is_cutout_line && j >= cutout {
                str.push_str(&cached.clone().on(COLOR_BAD).to_string());
            } else {
                str.push_str(cached);
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
    ignored: &[u32],
) -> DrawnArea {
    if values.is_empty() || max_width == 0 || max_height == 0 {
        return simple_rectangle(STR_EMPTY, max_width, max_height);
    }

    if values.len() > max_height as usize {
        let (data, ignored) = downscale_to_biggest_factor(values, ignored, max_height);
        return bar_graph_horizontal(&data, max_width, max_height, cutout, &ignored);
    }

    let mut lines = Vec::with_capacity(max_height as usize);
    let max = u32::max(1, *values.iter().max().unwrap_or(&0));
    let len = values.len();
    let actual_height = max_height as usize / len * len;
    let factor = actual_height / len;
    let unit_width = f64::from(max_width) / f64::from(max);
    let cutout_line = (f64::from(cutout) * unit_width) as usize;

    for (index, &val) in values.iter().enumerate() {
        if ignored.contains(&(index as u32)) {
            let str = STR_EMPTY.repeat(max_width as usize).on(COLOR_EMPTY);
            for i in 0..factor {
                lines.push(str.to_string());
            }
            continue;
        }
        let mut color = COLOR_GOOD;
        if val >= cutout {
            color = COLOR_BAD;
        }
        let bar_len = (f64::from(val) * unit_width).trunc() as usize;
        let rem_len = max_width as usize - bar_len;
        let mut str = format!(
            "{}{}",
            STR_EMPTY.repeat(bar_len).on(color),
            STR_EMPTY.repeat(rem_len)
        );
        // add cutout line
        if cutout_line < max_width as usize {
            if cutout_line.cmp(&bar_len) == std::cmp::Ordering::Less {
                str = format!(
                    "{}{}{}{}",
                    STR_EMPTY.repeat(cutout_line).on(color),
                    STR_CUTOUT_VERT.with(COLOR_CUTOUT).on(color),
                    STR_EMPTY.repeat(bar_len - cutout_line - 1).on(color),
                    STR_EMPTY.repeat(rem_len)
                );
            } else {
                // fix with floating point math problem, which causes a bad
                // bar to not go over the cutout line!
                let mut cutout_str = STR_CUTOUT_VERT.with(COLOR_CUTOUT);
                if color == COLOR_BAD {
                    cutout_str = cutout_str.on(color);
                }
                str = format!(
                    "{}{}{}{}",
                    STR_EMPTY.repeat(bar_len).on(color),
                    STR_EMPTY.repeat(cutout_line - bar_len),
                    cutout_str,
                    STR_EMPTY.repeat(rem_len - (cutout_line - bar_len) - 1),
                );
            }
        }
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
    ignored: &[u32],
    label_shrink: bool,
) -> DrawnArea {
    const MIN_GRAPH_SIZE: usize = 3;
    if (max_height as usize) < values.len() {
        if label_shrink {
            let (data, ignored) = downscale_to_biggest_factor(values, ignored, max_height);
            return bar_graph_horizontal_label(
                &data,
                max_width,
                max_height,
                cutout,
                &ignored,
                label_shrink,
            );
        }
        return bar_graph_horizontal(values, max_width, max_height, cutout, ignored);
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
        return bar_graph_horizontal(values, max_width, max_height, cutout, ignored);
    }

    let factor = (max_height as usize) / values.len();
    let cached_left = STR_EMPTY.repeat(left_len);
    let cached_right = STR_EMPTY.repeat(right_len);

    let actual_max_width = max_width as usize - label_len;
    let mut graph =
        bar_graph_horizontal(values, actual_max_width as u32, max_height, cutout, ignored);
    let e1 = "----€ ".with(COLOR_EMPTY).bold();
    let e2 = "- ".with(COLOR_EMPTY).bold();
    let e3 = STR_EMPTY.repeat(right_len - 6);
    let e4 = STR_EMPTY.repeat(left_len - 2);
    for (index, &value) in values.iter().enumerate() {
        let mut color = COLOR_GOOD;
        if value >= cutout {
            color = COLOR_BAD;
        }
        let index_fmt = format!(" {:>2} ", index + 1).with(COLOR_TEXT).bold();
        let len = max_value.len();
        let tmp_fmt = format!("{:>len$}", format!("{:03}", values[index]));
        let tmp = tmp_fmt.len() - 2;
        let value_fmt = format!("{}.{}\u{20ac} ", &tmp_fmt[..tmp], &tmp_fmt[tmp..]);
        let value_fmt = value_fmt.with(color).bold();
        if let Some(line) = graph.area.get_mut(index * factor) {
            if ignored.contains(&(index as u32)) {
                let index_len = index_fmt.to_string().len();
                *line = format!("{e4}{e2}{line}{e3}{e1}");
            } else {
                *line = format!("{index_fmt}{line}{value_fmt}");
            }
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
        assert_eq!(vec![2, 7, 10], downscale_to_biggest_factor(&data, &[], 4).0);

        let data = [1, 3, 5, 0, 0, 7, 10, 0, 1];
        let ignored = [2, 3, 7];
        let expected = (vec![2, 0, 3, 10, 1], vec![1]);
        assert_eq!(expected, downscale_to_biggest_factor(&data, &ignored, 5));
    }
}
