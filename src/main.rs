use std::io;
use std::io::stdin;
use reqwest;
use serde_json::Value;
use tokio;

enum GuessError {
    NotFound,
}

fn main() {
    let mut health = 10;
    let word: String = match get_word() {
        Ok(str) => str,
        Err(_) => {
            println!("word from API is missing");
            return;
        }
    };

    let mut hash_word = hash_word(&word);

    let mut guessed_letters: Vec<char> = Vec::new();

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

        guessed_letters.push(guessed_letter);

        match check_guess(&mut guessed_letters, &word) {
            Ok(guess_word) => {
                println!("Correct!");
                hash_word = guess_word;
            }
            Err(_) => {
                health -= 1;
                println!("Incorrect! Health left {}", health);
            },
        }

        if is_win(&hash_word) {
            println!("You win!!!");
            println!("The word was {}", word);
            break;
        } else {
            if health <= 0 {
                println!("You lose the word was {}", word);
                break;
            }
            continue;
        }
    }
}

fn get_user_typed_letter() -> Result<String, io::Error> {
    println!("Enter a letter: ");

    let mut guessed_letter = String::new();

    stdin().read_line(&mut guessed_letter)?;

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

fn check_guess(guessed_letters: &mut Vec<char>, word: &String) -> Result<String, GuessError> {
    let last_guess = match guessed_letters.last() {
        Some(&c) => c,
        None => return Err(GuessError::NotFound),
    };

    if word.contains(last_guess) {
        let mut guess_hash_word: String = String::new();

        for el in word.chars() {
            if guessed_letters.contains(&el) {
                guess_hash_word.push(el);
            } else {
                guess_hash_word.push_str(" _ ");
            }
        }

        Ok(guess_hash_word)
    } else {
        guessed_letters.pop();
        Err(GuessError::NotFound)
    }
}

fn is_win(word: &String) -> bool {
    if !word.contains("_") {
        true
    } else {
        false
    }
}

#[tokio::main]
async fn get_word() -> Result<String, Box<dyn std::error::Error>> {

    let url = "https://random-word-api.vercel.app/api?words=1";

    let client = reqwest::Client::new();

    // Wysyłamy GET
    let res = client.get(url)
        .send()
        .await?;

    // Parsowanie odpowiedzi jako JSON
    let json: Value = res.json().await?;
    let word = json.get(0)
        .and_then(|c| c.as_str())
        .ok_or("The word from api is missing")?;

    return Ok(word.to_string());
}