fn main() {
    let graph = track_payments_rust::tui_renderer::templates::bar_graph_vertical(
        &[
            752, 707, 2787, 1019, 864, 890, 2853, 841, 989, 678, 990, 1812, 733, 714, 782, 931,
            1722, 1803, 862, 1278, 1079, 857, 558, 1450, 536, 857, 649,
        ],
        31 * 6,
        30,
        1000,
    );
    for line in graph {
        println!("{line}");
    }
}
