use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::nfa;

#[derive(Debug)]
pub struct DFA {
    alphabet: HashSet<String>,
    initial_state: String,
    states: HashMap<String, DFAState>,
}

#[derive(Debug)]
struct DFAState {
    is_final: bool,
    transitions: HashMap<String, String>,
}

impl DFA {
    fn transition(&self, from_state: &str, symbol: &str) -> Option<&String> {
        self.states.get(from_state).unwrap().transitions.get(symbol)
    }

    pub fn resolve_transitions(&self, symbols: Vec<&str>) -> Result<Option<String>, String> {
        let mut current_state: String = self.initial_state.clone();
        for symbol in symbols {
            // Check that symbol is in alphabet
            if !self.alphabet.contains(symbol) {
                return Err(format!("Symbol {} isn't the in alphabet", symbol));
            }
            // Compute next state
            let next_state = self.transition(current_state.as_str(), symbol);
            let next_state = match next_state {
                Some(s) => s.clone(),
                None => return Ok(None),
            };
            current_state = next_state;
        }
        Ok(Some(current_state))
    }

    pub fn is_this_state_final(&self, state: &str) -> bool {
        self.states.get(state).unwrap().is_final
    }
}

impl From<nfa::NFA> for DFA {
    fn from(original_automata: nfa::NFA) -> Self {
        let alphabet = original_automata.alphabet;

        unimplemented!()
        // TODO: Implement the conversion from NFA to DFA
    }
}

impl fmt::Display for DFA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Automata {{\n")?;
        write!(f, "    Language: {:?},\n", self.alphabet)?;
        write!(f, "    States: [\n")?;
        for (state_name, state) in &self.states {
            write!(f, "        {}: {:?}\n", state_name, state)?;
        }
        write!(f, "    ],\n")?;
        write!(f, "    Initial State: {},\n", self.initial_state)?;
        write!(f, "}}")
    }
}
