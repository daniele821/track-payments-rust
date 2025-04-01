#[must_use]
pub fn bar_graph_vertical(values: &[u32], width: u32, height: u32) -> Vec<String> {
    let max = *values.iter().max().unwrap_or(&0);
    let len = values.len();
    let unit_heigh = f64::from(height) / f64::from(max);
    let mut lines = Vec::with_capacity(height as usize);
    for i in (1..=height).rev() {
        let mut str = String::with_capacity(len);
        for j in values {
            if f64::from(i - 1) < f64::from(*j) * unit_heigh - 0.5 {
                str.push('@');
            } else {
                str.push(' ');
            }
        }
        lines.push(str);
    }
    lines
}
