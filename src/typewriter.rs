/*
Type text to the console with delay mimicing a typewriter.
 */

 use std:: { 
    // To print one char at a time and wait for user input
    io::{self, Write},

    // For printing delay
    thread, time::Duration  
};

// For printing delay 
use rand::Rng; 

/*
TODO: annie - maybe put this in its own file inside the typewrite mod?
- But commit before doing that ..
- add colors, for ful dialogue or by line
 */

pub type Dialogue = Vec<DialogueLine>;
pub struct DialogueLine {

    /// The text to print out to the console.
    text: String,

    /// `true` is the player must press enter to continue the dialogue at the end of this line.
    requires_confirmation: bool,

    /// Add Extra delay at the end of this line when printing it as part of a multiline dialogue.
    eol_delay: Option<u64>
}

impl DialogueLine {
    
    pub fn standard(text: &str) -> DialogueLine {
        DialogueLine {
            text: String::from(text),
            requires_confirmation: false,
            eol_delay: None
        }
    }

    pub fn with_delay(text: &str, eol_delay: u64) -> DialogueLine {
        DialogueLine { 
            text: String::from(text), 
            requires_confirmation: false, 
            eol_delay: Some(eol_delay)
        }
    }

    pub fn with_confirmation(text: &str) -> DialogueLine {
        DialogueLine { 
            text: String::from(text), 
            requires_confirmation: true, 
            eol_delay: None 
        }
    }
}

/// Print a full dialogue to the console in a typewriter-ish manner.
/// 
/// * `dialogue` A vector or dialogue lines
pub fn print_dialogue(dialogue: Dialogue) {

    for line in dialogue {
        print(&line.text);

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
}

/// Print text to the console in a typewrite-ish manner
/// 
/// * `text` - The text to print to the console
fn print(text: &str) {

    let char_list: Vec<char> = text.chars().collect();

    for char in char_list {
        random_wait(30, 100);
        
        print!("{}", char);
        
        let _ = io::stdout().flush();
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
