fn inverter_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original = "Rust é incrível!";
    let invertida = inverter_string(original);
    println!("Original: {}", original);
    println!("Invertida: {}", invertida);
}