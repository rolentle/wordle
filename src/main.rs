use std::io;
mod words;
use crate::words::*;

fn main() {
    let answer = ANSWERS[0];
        println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: String = guess.trim().parse().expect("Please type in a word");
    println!("guess: {}", &guess);
    let valid: bool = VALID_WORDS
        .iter()
        .map(|w| w.to_string())
        .collect::<String>()
        .contains(&guess);
    println!("valid word: {}", valid);
    println!("answer: {}", answer);
    println!("correct?: {}", answer == guess);
}
