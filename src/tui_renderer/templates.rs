#[must_use]
pub fn bar_graph_vertical(values: &[u32], width: u32, height: u32) -> Vec<String> {
    let max = *values.iter().max().unwrap_or(&0);
    let len = values.len();
    let unit_heigh = f64::from(height) / f64::from(max);
    let mut lines = Vec::with_capacity(height as usize);
    for i in 0..height {
        let mut str = String::with_capacity(len);
        for j in values {
            print!("{j},{i} | ");
            if f64::from(*j) * unit_heigh < f64::from(i + 1) {
                str.push('x');
            } else {
                str.push(' ');
            }
        }
        lines.push(str);
    }
    lines
}

#[cfg(test)]
mod tests {
    use crate::tui_renderer::templates::bar_graph_vertical;

    #[test]
    pub fn bar_graph() {
        let graph = bar_graph_vertical(&[1, 2, 3, 0, 2, 4, 1, 2], 8, 4);
        for line in graph {
            println!("{line}");
        }
    }
}
