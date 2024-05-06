pub type Lines = Vec<Line>;

pub struct Line {

    /// The text to print out to the console.
    pub(super) text: String,

    /// `true` is the player must press enter to continue the dialogue at the end of this line.
    pub(super) requires_confirmation: bool,

    /// Add extra delay at the end of this line when printing it as part of a multiline dialogue.
    pub(super) eol_delay: Option<u64>,

    /// Color to print just this line in
    pub(super) color: Option<LineColor>
}

impl Line {

    pub fn colored(text: &str, color: LineColor) -> Line {
        Line {
            text: String::from(text),
            requires_confirmation: false,
            eol_delay: None,
            color: Some(color)
        }
    }
    
    pub fn standard(text: &str) -> Line {
        Line {
            text: String::from(text),
            requires_confirmation: false,
            eol_delay: None,
            color: None
        }
    }

    pub fn with_delay(text: &str, eol_delay: u64) -> Line {
        Line { 
            text: String::from(text), 
            requires_confirmation: false, 
            eol_delay: Some(eol_delay),
            color: None
        }
    }

    pub fn with_confirmation(text: &str) -> Line {
        Line { 
            text: String::from(text), 
            requires_confirmation: true, 
            eol_delay: None,
            color: None
        }
    }
}
enum TextColor {
    Blue,
    Green,
    Red
}
pub struct LineColor {
    color: TextColor
}

impl LineColor {

    #[allow(dead_code)]
    pub fn blue() -> LineColor {
        LineColor { color: TextColor::Blue }
    }

    #[allow(dead_code)]
    pub fn green() -> LineColor {
        LineColor { color: TextColor::Green }
    }

    pub fn red() -> LineColor {
        LineColor { color: TextColor::Red }
    }

    pub(super) fn color_code(&self) -> &str {
        match self.color {
            TextColor::Blue =>  "\x1B[34m",
            TextColor::Red => "\x1B[31m",
            TextColor::Green => "\x1B[32m"
        }
    }

    pub(super) fn reset_code() -> &'static str {
        "\x1B[39m"
    }
}