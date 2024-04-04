fn shortest_word(sentence: &str) -> String {
    let mut shortest = String::new();
    let mut current_word = String::new();
    let mut shortest_length = usize::MAX;

    for c in sentence.chars() {
        if c == ' ' {
            if current_word.len() < shortest_length {
                shortest = current_word.clone();
                shortest_length = current_word.len();
            }
            current_word.clear();
        } else {
            current_word.push(c);
        }
    }

    // Check the last word
    if !current_word.is_empty() && current_word.len() < shortest_length {
        shortest = current_word;
    }

    shortest
}

fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let shortest = shortest_word(input.trim());

    println!("Shortest word in the string: {}", shortest);
}
