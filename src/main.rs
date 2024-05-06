mod typewriter;
use typewriter::DialogueLine;

fn main() {

    typewriter::print_dialogue(
        vec![
            DialogueLine::with_delay("Some dialogue that is printed and then waits a bit before printing the next line", 2000),
            DialogueLine::with_confirmation("Dialogue line 2, that will require use input before continuing to the next line"),
            DialogueLine::standard("Just some dialogue, nothing to see here, just a standard implementation")
        ]
    )
}
