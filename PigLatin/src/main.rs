use std::io;

fn converter (word: &String) -> String {
    let mut pig_word = String::from(&word[1..]);

    if ['a', 'e', 'i', 'o', 'u'].contains(&word.chars().nth(0).unwrap()) {
        pig_word.push_str("-h");
    }
    else {
        pig_word.push_str(&format!("-{}", word.chars().nth(0).unwrap()));
    }
    pig_word.push_str("ay");
    
    pig_word
}

fn main() {
    let mut latin_word = String::from("");

    println!("Insert a word to be converted in pig latin: ");

    io::stdin()
        .read_line(&mut latin_word)
        .expect("Failed to read line");
    latin_word.pop();

    println!("The word to convert is: {latin_word}");

    let pig_word = converter(&latin_word);

    println!("Successfully converted the word in pig latin: {pig_word}")

}
