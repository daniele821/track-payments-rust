fn main() {
    let graph = track_payments_rust::tui_renderer::templates::bar_graph_vertical(
        &[100, 200, 20, 0, 200, 400, 100, 212],
        100,
        30,
    );
    for line in graph {
        println!("{line}");
    }
}
