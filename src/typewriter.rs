/*
Type text to the console with delay mimicing a typewriter.
 */

pub mod dialogue;
use dialogue::LineColor;

use std:: { 
    // To print one char at a time and wait for user input
    io::{self, Write},

    // For printing delay
    thread, time::Duration  
};

// For printing delay 
use rand::Rng;

use self::dialogue::Line;

/// Print every line with the same configuration
pub fn print_dialogue_lines(text_lines: Vec<&str>, config: dialogue::LineConfig) {

    for text in text_lines {
        print_line(Line::configured(text, config));
    }
}

// TODO: - experiment with putting the typewriter.rs file inside the typewriter folder .. 

/// Print a full dialogue to the console in a typewriter-ish manner.
/// 
/// * `dialogue` A vector or dialogue lines
pub fn print_dialogue(dialogue: dialogue::Lines) {

    for line in dialogue {
        print_line(line);
    }
}

fn print_line(line: dialogue::Line) {
    
    print(&line.text, &line.color);

    if let Some(delay) = line.eol_delay {
        wait(delay);    
    }

    if line.requires_confirmation {
        
        print!("> ");
        let _ = io::stdout().flush(); // This line ensures that previous print appears

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong");
    }
}

/// Print text to the console in a typewrite-ish manner
/// 
/// * `text` - The text to print to the console
/// * `color` - Optional color to apply to the text being printed
fn print(text: &str, color: &Option<LineColor>) {

    let char_list: Vec<char> = text.chars().collect();

    if let Some(line_color) = color {
        let code = line_color.color_code();
        print!("{code}");
    }

    for char in char_list {
        random_wait(30, 100);

        print!("{}", char);
        
        let _ = io::stdout().flush();
    }

    if color.is_some() {
        let stop = LineColor::reset_code();
        print!("{stop}");
    }

    print!("\n");
}

/// Put the program to sleep for a random amount of time.
/// 
/// Use this for a more dynamic delay when printing text to the screen.
/// 
/// * `longest` - The longest the program will wait
/// * `shortest` - The shortest it will wait
fn random_wait(shortest: u64, longest: u64) {
    let mut rand_range = rand::thread_rng();
    let time_to_wait = rand_range.gen_range(shortest..longest + 1);
    wait(time_to_wait);
}

/// Put the program to sleep for the given amount of milliseconds
/// 
/// * `millisec` - The amount of milliseconds the program should sleep for before continuing
fn wait(millisec: u64) {
    thread::sleep(Duration::from_millis(millisec));
}