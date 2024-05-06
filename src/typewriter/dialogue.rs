
pub type Lines = Vec<Line>;

pub struct Line {

    /// The text to print out to the console.
    pub(super) text: String,

    /// `true` is the player must press enter to continue the dialogue at the end of this line.
    pub(super) requires_confirmation: bool,

    /// Add Extra delay at the end of this line when printing it as part of a multiline dialogue.
    pub(super) eol_delay: Option<u64>
}

impl Line {
    
    pub fn standard(text: &str) -> Line {
        Line {
            text: String::from(text),
            requires_confirmation: false,
            eol_delay: None
        }
    }

    pub fn with_delay(text: &str, eol_delay: u64) -> Line {
        Line { 
            text: String::from(text), 
            requires_confirmation: false, 
            eol_delay: Some(eol_delay)
        }
    }

    pub fn with_confirmation(text: &str) -> Line {
        Line { 
            text: String::from(text), 
            requires_confirmation: true, 
            eol_delay: None 
        }
    }
}