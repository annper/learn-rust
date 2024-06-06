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

#[derive(Copy, Clone)]
pub struct LineConfig {
    pub requires_confirmation: bool,
    pub eol_delay: Option<u64>,
    pub color: Option<LineColor>
}

impl Line {

    pub fn configured(text: &str, config: LineConfig) -> Line {
        Line { 
            text: String::from(text), 
            requires_confirmation: config.requires_confirmation, 
            eol_delay: config.eol_delay, 
            color: config.color 
        }
    }

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
#[derive(Debug, PartialEq, Copy, Clone)]
enum TextColor {
    Blue,
    Green,
    Red
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use crate::typewriter::dialogue::TextColor;

    use super::{Line, LineColor, LineConfig};

    #[test]
    fn colored_line_is_configured_correctly() {
        
        let line_color = LineColor::blue();
        let colored_line = Line::colored("text", line_color);

        assert_eq!(colored_line.color.expect("value"), line_color);
        assert_eq!(colored_line.text, "text");
        assert_eq!(colored_line.eol_delay, None);
        assert_eq!(colored_line.requires_confirmation, false);
    }

    #[test]
    fn standard_line_is_configured_correctly() {

        let test_text = "hello";

        let standard_line = Line::standard(test_text);

        assert_eq!(standard_line.color, None);
        assert_eq!(standard_line.text, test_text);
        assert_eq!(standard_line.eol_delay, None);
        assert_eq!(standard_line.requires_confirmation, false);
    }

    #[test]
    fn line_with_delay_is_configured_correctly() {

        let test_text = "hello";

        let delayed_line = Line::with_delay(test_text, 1);

        assert_eq!(delayed_line.color, None);
        assert_eq!(delayed_line.text, test_text);
        assert_eq!(delayed_line.eol_delay.expect("value"), 1);
        assert_eq!(delayed_line.requires_confirmation, false);
    }

    #[test]
    fn line_with_confirmation_is_configured_correctly() {

        let test_text = "hello";

        let confirm_line = Line::with_confirmation(test_text);

        assert_eq!(confirm_line.color, None);
        assert_eq!(confirm_line.text, test_text);
        assert_eq!(confirm_line.eol_delay, None);
        assert!(confirm_line.requires_confirmation);
    }

    #[test]
    fn configured_line_retains_configuration() {

        let test_text = "hello";

        let config = LineConfig {
            requires_confirmation: true,
            eol_delay: Some(1000),
            color: Some(LineColor::blue())
        };

        let configed_line = Line::configured(test_text, config);

        assert_eq!(configed_line.text, test_text);
        assert_eq!(configed_line.color.expect("value"), config.color.expect("value"));
        assert_eq!(configed_line.eol_delay.expect("value"), config.eol_delay.expect("value"));
        assert!(configed_line.requires_confirmation);

    }

    #[test]
    fn line_color_maps_to_correct_text_color() {
        assert_eq!(LineColor::blue().color, TextColor::Blue);
        assert_eq!(LineColor::green().color, TextColor::Green);
        assert_eq!(LineColor::red().color, TextColor::Red);
    }
}