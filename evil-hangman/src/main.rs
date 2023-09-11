use rand::Rng;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

  
fn main() {
    let title = r###"
     ______     _ _   _    _                                         
    |  ____|   (_) | | |  | |                                        
    | |____   ___| | | |__| | __ _ _ __   __ _ _ __ ___   __ _ _ __  
    |  __\ \ / / | | |  __  |/ _` | '_ \ / _` | '_ ` _ \ / _` | '_ \ 
    | |___\ V /| | | | |  | | (_| | | | | (_| | | | | | | (_| | | | |
    |______\_/ |_|_| |_|  |_|\__,_|_| |_|\__, |_| |_| |_|\__,_|_| |_|
                                          __/ |                      
                                         |___/                       
                                                  Made in Rust with <3"###;
    println!("{}\n", title);

    let mut rng = rand::thread_rng();

    let word_length: usize = rng.gen_range(3..11) as usize;

    let mut possible_words: Vec<String> = vec![];
    let mut letters_guessed: Vec<char> = Vec::with_capacity(26);
    let mut letters_revealed: Vec<char> = vec!['_'; word_length]; 

    populate_possible_words(&mut possible_words, word_length, "./dictionary.txt".to_string());

    println!("This word is {} letters long.\n", word_length);

    let mut running = true;
    while running {
        println!("\nEnter a guess:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to readline.");
        let guess = input.trim().to_lowercase();
        
        if guess.chars().all(|c| matches!(c, 'a'..='z')) {
            if guess.len() == 1 {
                let letter = guess.chars().nth(0).unwrap();
                if !letters_guessed.contains(&letter) {
                    letters_guessed.push(letter);
                    trim_possible_words(&mut possible_words, letter)
                } else {
                    println!("You've already guessed {}", letter.to_uppercase());
                }
            } else {
                println!("You must guess one letter");
            }
        } else {
            println!("Your guess must be a letter");
        }
    }
}

fn trim_possible_words(possible_words: &mut Vec<String>, guess: char) {

}

fn populate_possible_words(possible_words: &mut Vec<String>, word_length: usize, dictionary_path: String) {
    let dictionary = File::open(dictionary_path).unwrap();
    let dictionary_reader = BufReader::new(dictionary);
    for line in dictionary_reader.lines() {
        let word = line.unwrap();
        if word.len() == word_length {
            possible_words.push(word);
        }
    }
}
