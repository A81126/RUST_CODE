use std::io;

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_str = strings[0];

    let mut prefix = String::new();

    'outer: for (i, c) in first_str.chars().enumerate() {
        for string in &strings[1..] {
            if i >= string.len() || string.chars().nth(i).unwrap() != c {
                break 'outer;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    println!("Enter a set of strings separated by space:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let strings: Vec<&str> = input.split_whitespace().collect();

    let prefix = longest_common_prefix(&strings);

    if prefix.is_empty() {
        println!("No common prefix found.");
    } else {
        println!("Longest common prefix: {}", prefix);
    }
}
