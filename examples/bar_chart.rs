fn main() {
    let width = crossterm::terminal::size().unwrap().0;
    let width = u32::from(width);
    let height = 30;
    let graph = track_payments_rust::tui_renderer::templates::bar_graph_vertical(
        &[
            0, 752, 707, 2787, 1019, 864, 890, 2853, 0, 0, 841, 989, 678, 990, 1812, 0, 733, 714,
            782, 931, 1722, 1803, 862, 1278, 1079, 857, 558, 1450, 536, 857, 649,
        ],
        width,
        height,
        1000,
    );
    for line in graph {
        println!("{line}");
    }
    assert!(track_payments_rust::tui_renderer::templates::are_sizes_valid(&graph, width, height));
}
