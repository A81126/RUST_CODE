fn reverse_string(input: &str) -> String {
    let mut reversed = String::new();

    for ch in input.chars().rev() {
        reversed.push(ch);
    }

    reversed
}

fn main() {
    let original = "Hello, World!";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
