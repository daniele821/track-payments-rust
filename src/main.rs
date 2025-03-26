mod crypto;
mod payments;

fn main() {
    let i = strsim::jaro_winkler("HI there", "hi there");
    println!("{i}");
}
