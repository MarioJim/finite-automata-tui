use std::convert::TryFrom;
use std::env;
use std::fs;

mod dfa;
mod nfa;
mod tui;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file =
        fs::read_to_string(filename).expect(format!("Couldn't read file {}", filename).as_ref());
    let automata_struct = nfa::NFA::try_from(file).unwrap();
    println!("{}", automata_struct);

    tui::show_tui(automata_struct);
}
