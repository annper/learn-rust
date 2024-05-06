mod typewriter;

fn main() {

    typewriter::print_dialogue(
        vec![
            typewriter::dialogue::Line::with_delay("Some dialogue that is printed and then waits a bit before printing the next line", 2000),
            typewriter::dialogue::Line::with_confirmation("Dialogue line 2, that will require use input before continuing to the next line"),
            typewriter::dialogue::Line::standard("Just some dialogue, nothing to see here, just a standard implementation")
        ]
    )
}
