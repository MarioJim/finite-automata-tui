use std::convert::TryFrom;
use std::env;
use std::fs;
use tuikit::prelude::*;

mod automata;
mod input_field;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file =
        fs::read_to_string(filename).expect(format!("Couldn't read file {}", filename).as_ref());
    let automata_struct = automata::Automata::try_from(file).unwrap();
    println!("{}", automata_struct);

    // Interactive terminal
    let instructions = "Write the symbols separated by a space, press Esc or Ctrl+C to quit";
    let mut input_f = input_field::InputField::new();
    let term = Term::with_height(TermHeight::Fixed(4)).unwrap();
    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();
        // Match event
        match ev {
            Event::Key(Key::ESC) | Event::Key(Key::Ctrl('c')) => break,
            Event::Key(k) => input_f.receive_event(k),
            _ => {}
        }
        // Print instructions
        let _ = term.print(0, 0, instructions);
        // Print current states
        let final_states = automata_struct.resolve_transitions(input_f.get_split_value());
        let is_accepted = match final_states {
            Ok(v) => {
                let _ = term.print(1, 0, format!("{:?}", v).as_str());
                automata_struct.is_any_state_final(v)
            }
            Err(e) => {
                let attr = Attr {
                    fg: Color::RED,
                    ..Attr::default()
                };
                let _ = term.print_with_attr(1, 0, e.as_str(), attr);
                false
            }
        };
        // Print if string is accepted in the language
        let is_accepted = format!(
            "That string {} accepted by the automata",
            if is_accepted { "is" } else { "isn't" }
        );
        let _ = term.print(2, 0, is_accepted.as_str());
        // Print input
        input_f.print_in_term(&term, 3);
        // Display
        let _ = term.present();
    }
}
