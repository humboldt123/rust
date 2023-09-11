extern crate itertools;

use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use rand::Rng;
use itertools::Itertools;
  
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
    let mut guesses_taken = 0;
    populate_possible_words(&mut possible_words, word_length, "./dictionary.txt".to_string());

    println!("The target word is {} letters long.\n", word_length);

    let mut running = true;
    while running {
        if !letters_revealed.contains(&'_') {
            running = false;
            println!("\n\n\n{}\n\n", letters_revealed.iter().join(" "));
            match guesses_taken {
                26 => println!("Congratulations! You tried each letter in the alphabet until you got the word."),
                25 => println!("Congratulations! You tried almost every letter, but you eventually got there."),
                _ => println!("Congratulations! You guessed the word in {} guesses.", guesses_taken),
            }
        } else {
            println!("\n\n\n{}\n\nGUESS A LETTER\n", letters_revealed.iter().join(" "));

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to readline.");
            let guess = input.trim().to_lowercase();
            
            if guess.chars().all(|c| matches!(c, 'a'..='z')) {
                if guess.len() == 1 {
                    let letter = guess.chars().nth(0).unwrap();
                    if !letters_guessed.contains(&letter) {
                        letters_guessed.push(letter);
                        trim_possible_words(&mut possible_words, letter, &mut letters_revealed);
                        guesses_taken += 1;
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

fn trim_possible_words(possible_words: &mut Vec<String>, guess: char, letters_revealed: &mut Vec<char>) {
    // Create a hashmap of word groups
    let mut word_groups: HashMap<String, Vec<String>> = HashMap::new();
    
    // Add each word in our list to its corresponding group
    for word in &mut *possible_words {
        let group = get_string_group(word.to_string(), guess);
        word_groups.entry(group.clone()).or_insert_with(Vec::new).push(word.to_string());
    }

    // Get the largest group (key, value)
    let (biggest_group, word_list) = word_groups.iter().max_by(|(_, v1), (_, v2)| v1.len().cmp(&v2.len())).unwrap();
    *possible_words = word_list.clone();
    
    if !biggest_group.contains(guess) {
        println!("Sorry! The target word does NOT contain an {}", guess.to_uppercase());
    } else {
        // Add each instance of the letter to our letters revealed
        let mut appearances = 0;
        for (i, letter) in biggest_group.char_indices() {
            if letter == guess {
                letters_revealed[i] = letter;
                appearances += 1;
            }
        }
        println!(
            "Nice! The target word contains {}{}{}!",
            if appearances == 1 {"an ".to_string()} else {"".to_string()},
            guess.to_uppercase(),
            if appearances == 1 {"".to_string()} else {format!(" {} times", appearances)}
        )
    }
}

fn get_string_group(word: String, letter: char) -> String {
    word.chars().map(|c| if c == letter { c } else { '_' }).collect::<String>()
}