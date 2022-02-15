use std::io;

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
}

fn render_next_guess_placeholder() {
    println!("[][][][][]")
}

fn enter_guess() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");
    return guess;
}
