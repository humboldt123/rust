use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

use rand::Rng;
use itertools::Itertools;
use inline_colorization::*;

fn main() {
    let title = format!(r###"
     ______     _ _   _    _                                         
    |  ____|   (_) | | |  | |                                        
    | |____   ___| | | |__| | __ _ _ __   __ _ _ __ ___   __ _ _ __  
    |  __\ \ / / | | |  __  |/ _` | '_ \ / _` | '_ ` _ \ / _` | '_ \ 
    | |___\ V /| | | | |  | | (_| | | | | (_| | | | | | | (_| | | | |
    |______\_/ |_|_| |_|  |_|\__,_|_| |_|\__, |_| |_| |_|\__,_|_| |_|
                                          __/ |                      
                                         |___/                       
                                                  Made with {color_red}<3{color_reset} in Rust"###);
    println!("{}\n", title);

    let mut rng = rand::thread_rng();

    let word_length: usize = rng.gen_range(3..11) as usize;

    let mut possible_words: Vec<String> = vec![];
    let mut letters_guessed: Vec<char> = Vec::with_capacity(26);
    let mut letters_revealed: Vec<char> = vec!['_'; word_length]; 
    let mut guesses_taken = 0;
        
    populate_possible_words(&mut possible_words, word_length, "./dictionary.txt".to_string());

    println!("The target word is {} letters long.\n", word_length);

    // Main game loop
    loop {
        if !letters_revealed.contains(&'_') {
            println!("\n\n\n{}\n\n", letters_revealed.iter().join(" "));
            match guesses_taken {
                26 => println!("Congratulations! You guessed every single letter of the alphabet until you got to the word. (26 guesses)"),
                20..=25 => println!("Congratulations! You almost guessed every single letter, but you got there. ({} guesses)", guesses_taken),
                16..=19 => println!("Nice! You guessed the word in under 20 guesses! ({} guesses)", guesses_taken),
                10..=15 => println!("Impressive! You guessed the word in {} guesses! That's really {color_magenta}spectacular!{color_reset}", guesses_taken),
                _ => println!("Very impressive! You guessed the word in {} guesses. That's in the single digits!", guesses_taken),
            }
            break;
        } else {
            println!("\n\n{}\nGUESS A LETTER\n", letters_revealed.iter().join(" "));
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to readline.");
            let guess = input.trim().to_lowercase();
            // clear user input from the terminal
            print!("\x1B[1A\x1B[2K");


            if guess.chars().all(|c| matches!(c, 'a'..='z')) {
                if guess.len() == 1 {
                    let letter = guess.chars().nth(0).unwrap();
                    if !letters_guessed.contains(&letter) {
                        letters_guessed.push(letter);
                        trim_possible_words(&mut possible_words, letter, &mut letters_revealed);
                        guesses_taken += 1;
                    } else {
                        println!("You've already guessed {color_magenta}{}{color_reset}", letter.to_uppercase());
                        println!("\n{color_yellow}Hint!{color_reset} You can guess the following letters:\n{}", ('a'..='z').collect::<Vec<char>>().iter().filter(|letter| !letters_guessed.contains(letter)).join(" ").to_uppercase())
                    }
                } else {
                    println!("You must guess one letter");
                }
            } else {
                println!("Your guess must be a letter");
            }
            println!("------------------------------------------------");
        }
    }
}

/// Add each word in the dictionary of word_length the list of possible words
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

/// Set the list of possible words to the largest group of words with regard to the users guess
///
/// For example, if the list of possible words is ["ate", "ape", "all", "pin"] and the user
/// guesses 'a', the new list of possible words becomes ["ate", "ape", "all"] because there are
/// more words in the "a__" word family than the "___" word family.
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
        println!("Sorry! The target word does NOT contain an {color_magenta}{}{color_reset}", guess.to_uppercase());
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
            "Nice! The target word contains {}{color_yellow}{}{color_reset}{}!",
            if appearances == 1 {"an ".to_string()} else {"".to_string()},
            guess.to_uppercase(),
            if appearances == 1 {"".to_string()} else {format!(" {} times", appearances)}
        )
    }
}

/// Get the "word family" of a particular word with regard to a letter guess
///
/// For example; the word "apple" with the guess 'a' would return "a____", whereas
/// the word "banana" with the guess 'a' would return "_a_a_a".
fn get_string_group(word: String, letter: char) -> String {
    word.chars().map(|c| if c == letter { c } else { '_' }).collect::<String>()
}