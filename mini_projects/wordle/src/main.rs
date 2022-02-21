use colorful::Color;
use colorful::Colorful;
use std::io;

// TODO
// - use constant for get_result length. usize?
// validate input (length, chars)
// random word
// block fake words?
// use &str instead of String for guess?

#[derive(Copy, Clone)]
enum LetterResult {
    Correct,
    WrongLocation,
    Wrong,
}

struct GuessResult {
    guess: String,
    result: [LetterResult; WORD_LENGTH],
}

const MAX_GUESSES: usize = 6;
const WORD_LENGTH: usize = 5;

fn main() {
    let word = "CANAL";

    let mut guess_results: Vec<GuessResult> = Vec::new();

    for _ in 0..MAX_GUESSES {
        render_screen(&guess_results);

        let guess = enter_guess();
        let result = get_result(&guess, word);
        let guess_result = GuessResult { guess, result };
        guess_results.push(guess_result);
        if result.iter().all(|&i| matches!(i, LetterResult::Correct)) {
            println!("Woohoo!");
            break;
        }
    }
}

fn render_screen(guess_results: &Vec<GuessResult>) {
    // clear screen
    print!("{}[2J", 27 as char);

    println!("");
    println!("==============");
    println!("----WORDLE----");
    println!("==============");
    println!("");
    println!("Welcome to Wordle. Please type a 5 letter word and hit enter.");
    println!("");
    println!("Guess {}/{}", guess_results.len() + 1, MAX_GUESSES);
    println!("");

    for guess_result in guess_results {
        print_result(guess_result);
    }
}

fn get_result(guess: &String, word: &str) -> [LetterResult; WORD_LENGTH] {
    let mut result: [LetterResult; WORD_LENGTH] = [LetterResult::Wrong; WORD_LENGTH];
    let guess_uppercase = guess.to_uppercase();
    for i in 0..WORD_LENGTH {
        let guess_letter = guess_uppercase.chars().nth(i).unwrap();
        let word_letter = word.chars().nth(i).unwrap();
        if guess_letter == word_letter {
            result[i] = LetterResult::Correct;
        } else if word.contains(guess_letter) {
            result[i] = LetterResult::WrongLocation;
        }
    }
    return result;
}

fn print_result(guess_result: &GuessResult) {
    for i in 0..WORD_LENGTH {
        let guess_upper = guess_result.guess.to_uppercase();
        let guess_char_str = &guess_upper[i..i + 1];
        match guess_result.result[i] {
            LetterResult::Correct => {
                print!(
                    "{}",
                    guess_char_str.color(Color::DarkBlue).bg_color(Color::Green)
                )
            }
            LetterResult::WrongLocation => {
                print!(
                    "{}",
                    guess_char_str
                        .color(Color::DarkBlue)
                        .bg_color(Color::Orange3)
                )
            }
            LetterResult::Wrong => {
                print!(
                    "{}",
                    guess_char_str
                        .color(Color::DarkBlue)
                        .bg_color(Color::Grey42)
                )
            }
        }
    }
    println!("");
}

fn enter_guess() -> String {
    let guess = loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");
        guess = guess.trim().to_string();
        println!("{}", guess.chars().count());
        if guess.chars().count() == WORD_LENGTH {
            break guess;
        } else {
            println!("Guess must be {} letters long!", WORD_LENGTH);
        }
    };
    println!("{}", guess);
    return guess;
}
