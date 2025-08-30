# Hangman CLI (Rust)

A simple command-line **Hangman** game written in Rust. Guess the letters of a hidden word before you run out of health!

## Features

* Random English word selection (via API or offline word list)
* Reveals correct letters as you guess
* Tracks remaining health (lives)
* Simple and interactive CLI interface

## How to Play

1. Run the game:

```bash
cargo run
```

2. The game will display underscores (`_`) representing the hidden word.
3. Enter a letter guess.
4. If the letter is in the word, it will be revealed.
5. You lose health for incorrect guesses.
6. The game ends when:

   * You guess the full word → **You win!**
   * You run out of health → **You lose!**

## Requirements

* Rust (1.70+ recommended)
* Internet connection if using API for random words
* On Linux, you may need to install dependencies for Rust TLS support:

```bash
sudo apt update
sudo apt install pkg-config libssl-dev
```

## Random Words API Setup (Optional)

You can use the [Random Word API](https://random-word-api.vercel.app/api?words=1) to get random English words.

* Send a GET request to the URL:
  `https://random-word-api.vercel.app/api?words=1`
* The API returns a JSON array with a single word, e.g., `["example"]`.
* Parse it in Rust using `reqwest` and `serde_json` to get the word as a string.

## Run Example

```
Word: _ _ _ _ _
Guess a letter: a
Correct! Word: _ a _ _ _
Guess a letter: e
Incorrect! Health left: 4
```
