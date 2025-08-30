use rand::Rng;
use std::io;
use std::io::stdin;

enum GuessError {
    NotFound,
}

fn main() {
    let words: [String; 3] = ["essa".to_string(), "jaja".to_string(), "dupa".to_string()];

    // Get random word from words array
    let rand_word_index = rand::rng().random_range(0..2);
    let word: &String = &words[rand_word_index];

    // Hash word
    let mut hash_word = hash_word(word);

    loop {
        println!("The word: {}", hash_word);

        let guessed_letter: char = loop {
            match get_user_typed_letter() {
                Ok(str) => {
                    if str.trim().len() == 1 {
                        break str.trim().chars().next().unwrap();
                    } else {
                        println!("Provide ONLY one letter try one more");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Something went wrong try again");
                    continue;
                }
            };
        };

        match check_guess(guessed_letter, word) {
            Ok(guess_word) => {
                println!("Nice! you guess a letter");
                hash_word = guess_word;
            }
            Err(_) => println!("The word don't contains guessed letter try again"),
        }
    }
}

fn get_user_typed_letter() -> Result<String, io::Error> {
    println!("Enter a letter: ");

    let mut guessed_letter = String::new();

    stdin().read_line(&mut guessed_letter)?;
    let guessed_letter = guessed_letter.to_lowercase();
    return Ok(guessed_letter);
}

fn hash_word(word: &String) -> String {
    let number_of_letters = word.len();
    let mut hash_word = String::new();

    for i in 1..=number_of_letters {
        if i == number_of_letters {
            hash_word.push_str("_");
            break;
        }
        hash_word.push_str("_ ");
    }

    hash_word
}

fn check_guess(guess: char, word: &String) -> Result<String, GuessError> {
    if word.to_lowercase().contains(guess) {
        let mut guessed_hash_word = String::new();
        for el in word.chars() {
            if el == guess {
                guessed_hash_word.push(el);
                continue;
            }

            guessed_hash_word.push("_".chars().next().unwrap());
        }

        Ok(guessed_hash_word)
    } else {
        Err(GuessError::NotFound)
    }
}
