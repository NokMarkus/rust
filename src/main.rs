use rand::Rng;
use std::io;
use arboard::Clipboard;

fn main() {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let special_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?";

    let all_chars = format!("{lowercase}{uppercase}{numbers}{special_chars}");
    println!("    ");

    println!("Enter the number of characters for the random string:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    println!("  ");

    let length: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid number.");
            return;
        }
    };

    let mut rng = rand::thread_rng();
    let random_password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..all_chars.len());
            all_chars.chars().nth(idx).unwrap()
        })
        .collect();

    println!("Generated random password: {}", random_password);

    println!("   ");

    println!("Type the letter 'c', to copy the generated password. Type any other letter to EXIT:");

    let mut decision = String::new();
    io::stdin()
        .read_line(&mut decision)
        .expect("Failed to read input");

        println!("   ");

    if decision.trim().eq_ignore_ascii_case("c") {
        match copy_to_clipboard(&random_password) {
            Ok(_) => println!("The string has been copied to your clipboard."),
            Err(e) => eprintln!("Failed to copy to clipboard: {}", e),
        }
    } else {
        println!("Goodbye!");
    }
}

fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text.to_string()).map_err(|e| e.to_string())
}
