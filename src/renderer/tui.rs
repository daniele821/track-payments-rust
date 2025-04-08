use crossterm::style::{Color, Stylize};

const COLOR_TEXT: Color = Color::White;
const COLOR_GOOD: Color = Color::DarkGreen;
const COLOR_BAD: Color = Color::DarkRed;
const COLOR_EMPTY: Color = Color::DarkGrey;
const COLOR_CUTOUT: Color = Color::Yellow;

// https://en.wikipedia.org/wiki/Box-drawing_characters
const STR_EMPTY: &str = " ";
const STR_CUTOUT_VERT: &str = "╏";

fn downscale_to_biggest_factor(
    values: &[u32],
    ignored: &[u32],
    max_length: u32,
) -> (Vec<u32>, Vec<u32>, u32) {
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
            compacted_values.push(buffer.iter().sum());
        }
        buffer.clear();
    }

    (compacted_values, compacted_ignored, scaling_factor as u32)
}

pub fn simple_rectangle(elem: &str, width: u32, height: u32) -> Vec<String> {
    let mut lines = Vec::with_capacity(height as usize);
    let empty_line = elem.repeat(width as usize);
    for _ in 0..height {
        lines.push(empty_line.clone());
    }
    lines
}

pub fn bar_graph_horizontal(
    values: &[u32],
    max_width: u32,
    max_height: u32,
    cutout: f64,
    ignored: &[u32],
) -> Vec<String> {
    if values.is_empty() || max_width == 0 || max_height == 0 {
        return simple_rectangle(STR_EMPTY, max_width, max_height);
    }

    if values.len() > max_height as usize {
        let (data, ignored, factor) = downscale_to_biggest_factor(values, ignored, max_height);
        return bar_graph_horizontal(
            &data,
            max_width,
            max_height,
            cutout * factor as f64,
            &ignored,
        );
    }

    let mut lines = Vec::with_capacity(max_height as usize);
    let cutout_u32 = cutout as u32;
    let max = u32::max(cutout_u32, u32::max(1, *values.iter().max().unwrap_or(&0)));
    let len = values.len();
    let actual_height = max_height as usize / len * len;
    let factor = actual_height / len;
    let unit_width = f64::from(max_width) / f64::from(max);
    let mut cutout_line = (cutout * unit_width) as usize;

    if cutout_line == max_width as usize && max == cutout_u32 {
        cutout_line -= 1;
    }

    for (index, &val) in values.iter().enumerate() {
        if ignored.contains(&(index as u32)) {
            let str = STR_EMPTY.repeat(max_width as usize).on(COLOR_EMPTY);
            for _ in 0..factor {
                lines.push(str.to_string());
            }
            continue;
        }
        let mut color = COLOR_GOOD;
        if val >= cutout_u32 {
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
        for _ in 0..factor {
            lines.push(str.clone());
        }
    }

    lines
}

pub fn bar_graph_horizontal_label(
    values: &[u32],
    max_width: u32,
    max_height: u32,
    cutout: f64,
    ignored: &[u32],
) -> Vec<String> {
    bar_graph_horizontal_label_(values, max_width, max_height, cutout, ignored, 1)
}

fn bar_graph_horizontal_label_(
    values: &[u32],
    max_width: u32,
    max_height: u32,
    cutout: f64,
    ignored: &[u32],
    downscaling_factor: u32,
) -> Vec<String> {
    const MIN_GRAPH_SIZE: usize = 3;

    if values.is_empty() || max_width == 0 || max_height == 0 {
        return simple_rectangle(STR_EMPTY, max_width, max_height);
    }

    if (max_height as usize) < values.len() {
        let (data, ignored, factor) = downscale_to_biggest_factor(values, ignored, max_height);
        return bar_graph_horizontal_label_(
            &data,
            max_width,
            max_height,
            cutout * factor as f64,
            &ignored,
            factor,
        );
    }

    let max_index_len = (values.len() - 1) * downscaling_factor as usize + 1;
    let max_index_len = max_index_len.to_string().len();
    let left_len = max_index_len + 2;

    let max_value = format!("{:03}", values.iter().max().unwrap_or(&0));
    let len = max_value.len();
    let max_value = format!("{}.{}", &max_value[..len - 2], &max_value[len - 2..]);
    let max_value_len = max_value.len() + 1;
    let right_len = max_value_len + 2;

    let label_len = left_len + right_len;
    let cutout_u32 = cutout as u32;

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
        if value >= cutout_u32 {
            color = COLOR_BAD;
        }
        let index_val = index * downscaling_factor as usize + 1;
        let index_fmt = format!(" {index_val:>max_index_len$} ",)
            .with(COLOR_TEXT)
            .bold();
        let len = max_value.len();
        let tmp_fmt = format!("{:>len$}", format!("{:03}", values[index]));
        let tmp = tmp_fmt.len() - 2;
        let value_fmt = format!("{}.{}€ ", &tmp_fmt[..tmp], &tmp_fmt[tmp..]);
        let value_fmt = value_fmt.with(color).bold();
        if let Some(line) = graph.get_mut(index * factor) {
            if ignored.contains(&(index as u32)) {
                *line = format!("{e4}{e2}{line}{e3}{e1}");
            } else {
                *line = format!("{index_fmt}{line}{value_fmt}");
            }
        }
        for i in 1..factor {
            if let Some(line) = graph.get_mut(index * factor + i) {
                *line = format!("{cached_left}{line}{cached_right}");
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::{bar_graph_horizontal, bar_graph_horizontal_label, downscale_to_biggest_factor};

    #[test]
    pub fn downscale_data() {
        let data = [1, 3, 5, 9, 10];
        assert_eq!(
            vec![4, 14, 10],
            downscale_to_biggest_factor(&data, &[], 4).0
        );

        let data = [1, 3, 5, 0, 0, 7, 10, 0, 1];
        let ignored = [2, 3, 7];
        let expected = (vec![4, 0, 7, 10, 1], vec![1], 2);
        assert_eq!(expected, downscale_to_biggest_factor(&data, &ignored, 5));
    }

    #[test]
    pub fn horizontal_bar_chart() {
        let data = [1, 3, 5, 9, 10, 13, 15];
        let graph = bar_graph_horizontal(&data, 20, 10, 10.0, &[]);
        assert_eq!(graph.len(), 7);
        println!("{}", graph.join("\n"));
    }

    #[test]
    pub fn horizontal_bar_chart_label() {
        let data = [1, 3, 5, 9, 10, 13, 15];
        let graph = bar_graph_horizontal_label(&data, 20, 10, 10.0, &[]);
        assert_eq!(graph.len(), 7);
        println!("{}", graph.join("\n"));
    }
}
