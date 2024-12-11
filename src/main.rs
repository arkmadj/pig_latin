use std::io;

fn main() {
    println!("Please type a word");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let pig_latin = to_pig_latin(&input);
    println!("{}", pig_latin);
}

fn to_pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            if let Some(first_char) = chars.next() {
                if is_vowel(first_char) {
                    format!("{}-hay", word)
                } else {
                    let rest: String = chars.collect();
                    format!("{}-{}ay", rest, first_char)
                }
            } else {
                String::new()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_vowel(ch: char) -> bool {
    matches!(
        ch,
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
    )
}
