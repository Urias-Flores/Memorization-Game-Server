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
    println!("Memorize these characters:");
    println!("{}", chars);
    thread::sleep(Duration::from_secs(duration));
    clear_console();
    println!("Please type the characters you remember:");
}

fn calculate_timer_duration(difficulty: usize) -> u64 {
    match difficulty {
        3 => 3,   // Easy level: 3 seconds
        5 => 5,   // Medium level: 5 seconds
        7 => 7,   // Hard level: 7 seconds
        _ => 3,   // Default to 3 seconds for invalid difficulty (You can modify this as needed)
    }
}

fn clear_console() {
    let mut stdout = io::stdout();
    stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

fn main_menu() -> i32 {
    clear_console();
    println!("=====================================");
    println!("Welcome to the Memorization Game!");
    println!("=====================================");
    println!("1. Start Game");
    println!("2. Change Difficulty");
    println!("3. Exit");
    println!("Please choose an option: ");

    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        match choice.trim().parse::<i32>() {
            Ok(1) => return 1,
            Ok(2) => return 2,
            Ok(3) => return 3,
            _ => println!("Invalid choice. Please choose again:"),
        }
    }
}

fn choose_difficulty() -> usize {
    clear_console();
    println!("=====================================");
    println!("Difficulty Levels:");
    println!("1. Easy (3 characters)");
    println!("2. Medium (5 characters)");
    println!("3. Hard (7 characters)");
    println!("Please choose a difficulty level: ");

    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");
        match choice.trim().parse::<usize>() {
            Ok(1) => return 3,
            Ok(2) => return 5,
            Ok(3) => return 7,
            _ => println!("Invalid choice. Please choose again:"),
        }
    }
}

fn main() {
    const MAX_LEVEL: usize = 5;

    loop {
        match main_menu() {
            1 => {
                let difficulty = choose_difficulty();
                let mut current_level = 1;

                println!("=====================================");
                println!("Starting Game! Difficulty: Level {}", difficulty);

                while current_level <= MAX_LEVEL {
                    let characters_to_remember = generate_random_chars(difficulty);
                    clear_console();
                    println!("Level {}/{}:", current_level, MAX_LEVEL);
                    display_characters(&characters_to_remember, calculate_timer_duration(difficulty));

                    // Get player input
                    let mut player_input = String::new();
                    io::stdin().read_line(&mut player_input).expect("Failed to read input.");

                    // Compare input
                    let player_input = player_input.trim();
                    if player_input == characters_to_remember {
                        println!("Correct! You passed level {}.", current_level);
                        current_level += 1;
                        thread::sleep(Duration::from_secs(2));
                    } else {
                        println!("Incorrect! Game over.");
                        thread::sleep(Duration::from_secs(2));
                        break;
                    }
                }

                if current_level > MAX_LEVEL {
                    println!("=====================================");
                    println!("Congratulations! You completed all levels.");
                    println!("=====================================");
                }
            }
            2 => {
                clear_console();
                let difficulty = choose_difficulty();
                println!("=====================================");
                println!("Difficulty changed to Level {}.", difficulty);
                println!("=====================================");
                thread::sleep(Duration::from_secs(2));
            }
            3 => {
                clear_console();
                println!("=====================================");
                println!("Thank you for playing! Goodbye.");
                println!("=====================================");
                thread::sleep(Duration::from_secs(2));
                break;
            }
            _ => unreachable!(),
        }
    }
}
