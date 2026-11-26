
fn main() {
    println!("Rust in Bioinformatics");

    let sequence = "ATGATTCCTCATGAAGAGCCTGGATCCCCTACAGAAATCAAATGTGACTTTCCGTTTATCAGACTAAAATCAGAGCCATCCAGACAGTGA";

    let length = sequence.len();

    println!("DNA Sequence: {}", sequence);
    println!("Sequence Length: {} bp", length);
}
