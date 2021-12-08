fn main() {
    if let Err(e) = day8::run("data/input.txt") {
        eprintln!("Error opening the input file: {}", e);
        std::process::exit(1);
    };
}
