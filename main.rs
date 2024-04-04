fn is_palindrome(input: &str) -> bool {
    input.chars().eq(input.chars().rev())
}

fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim(); // Remove trailing newline and whitespaces

    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
