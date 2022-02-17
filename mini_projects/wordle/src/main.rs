use colorful::Color;
use colorful::Colorful;
use std::io;

// TODO
// - use constant for get_result length. usize?
// validate input (length, chars)

#[derive(Copy, Clone)]
enum LetterResult {
    Correct,
    WrongLocation,
    Wrong,
}

const MAX_GUESSES: usize = 6;
const WORD_LENGTH: usize = 5;

fn main() {
    let word = "CANAL";
    println!("");
    println!("==============");
    println!("----WORDLE----");
    println!("==============");
    println!("");
    println!("Welcome to Wordle. Please type a 5 letter word and hit enter.");

    for i in 0..MAX_GUESSES {
        render_next_guess_placeholder();
        let guess = enter_guess();
        println!("You guessed: {}", guess);
        let result = get_result(guess, word);
        print_result(result);
        println!("");
        if result.iter().all(|&i| matches!(i, LetterResult::Correct)) {
            break;
        }
    }
}

fn render_next_guess_placeholder() {
    println!("[][][][][]")
}

fn get_result(guess: String, word: &str) -> [LetterResult; WORD_LENGTH] {
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

fn print_result(result: [LetterResult; WORD_LENGTH]) {
    for letter_result in result {
        match letter_result {
            LetterResult::Correct => {
                print!("{}", "O".color(Color::DarkBlue).bg_color(Color::Green))
            }
            LetterResult::WrongLocation => {
                print!("{}", "o".color(Color::DarkBlue).bg_color(Color::Orange3))
            }
            LetterResult::Wrong => {
                print!("{}", "X".color(Color::DarkBlue).bg_color(Color::DarkGray))
            }
        }
    }
}

fn enter_guess() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");
    return guess;
}
