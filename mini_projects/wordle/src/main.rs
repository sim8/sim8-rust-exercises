use std::io;

// TODO
// - use constant for get_result length. usize?

#[derive(Copy, Clone)]
enum LetterResult {
    Correct,
    WrongLocation,
    Wrong,
}

const WORD_LENGTH: usize = 5;

fn main() {
    let word = "CANAL";
    println!("");
    println!("==============");
    println!("----WORDLE----");
    println!("==============");
    println!("");
    println!("Welcome to Wordle. Please type a 5 letter word and hit enter.");
    render_next_guess_placeholder();

    let guess = enter_guess();
    println!("You guessed: {}", guess);

    let result = get_result(guess, word);
    print_result(result);
    println!("");
}

fn render_next_guess_placeholder() {
    println!("[][][][][]")
}

fn get_result(guess: String, word: &str) -> [LetterResult; WORD_LENGTH] {
    let result: [LetterResult; WORD_LENGTH] = [LetterResult::Correct; WORD_LENGTH];
    return result;
}

fn print_result(result: [LetterResult; WORD_LENGTH]) {
    for letter_result in result {
        match letter_result {
            LetterResult::Correct => print!("O"),
            LetterResult::WrongLocation => print!("o"),
            LetterResult::Wrong => print!("X"),
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