mod typewriter;
use typewriter::dialogue::{self, LineColor};
fn main() {

    typewriter::print_dialogue(
        vec![
            typewriter::dialogue::Line::colored("Colorful dialogue!", LineColor::red()),
            typewriter::dialogue::Line::with_delay("Some dialogue that is printed and then waits a bit before printing the next line", 2000),
            typewriter::dialogue::Line::with_confirmation("Dialogue line 2, that will require use input before continuing to the next line"),
            typewriter::dialogue::Line::standard("Just some dialogue, nothing to see here, just a standard implementation")
        ]
    );

    let text_only_lines = vec![
        "Hello",
        "A little bit of a longer line with more information",
        "Oh and this too"
    ];

    typewriter::print_dialogue_lines(text_only_lines, dialogue::LineConfig {
        requires_confirmation: false,
        eol_delay: None,
        color: None
    });
}
