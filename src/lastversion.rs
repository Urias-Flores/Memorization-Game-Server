use std::io;
use rand::Rng;
use std::time::Duration;
use std::thread;
use crossterm::{ExecutableCommand, cursor};

fn generate_random_chars(length: usize) -> String {
    let chars: Vec<char> = (b'A'..=b'Z').map(char::from).collect();
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

fn display_characters(chars: &str, duration: u64) {
    print!("Memorize these characters: ");
    for num in (1..=3).rev() {
        print!("{}.. ", num);
        io::Write::flush(&mut io::stdout()).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!();
    println!("{}", chars);
    thread::sleep(Duration::from_secs(duration));
    clear_console();
    println!("Please type the characters you remember:");
}


fn clear_console() {
    let mut stdout = io::stdout();
    stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

fn main() {
    const MAX_LEVEL: usize = 5;
    const MEMORIZE_DURATION: u64 = 5; // Adjust the duration as you like (in seconds)

    let mut current_level = 1;

    println!("Welcome to the Memorization Game!");

    while current_level <= MAX_LEVEL {
        let characters_to_remember = generate_random_chars(current_level);
        clear_console(); // Clear the console before displaying characters
        display_characters(&characters_to_remember, MEMORIZE_DURATION);

        // Get player input
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("Failed to read input.");

        // Compare input
        let player_input = player_input.trim();
        if player_input == characters_to_remember {
            println!("Correct! You passed level {}.", current_level);
            current_level += 1;
        } else {
            println!("Incorrect! Game over.");
            break;
        }
    }

    if current_level > MAX_LEVEL {
        println!("Congratulations! You completed all levels.");
    }
}
