use multimap::MultiMap;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug)]
pub struct NFA {
    alphabet: HashSet<String>,
    initial_state: String,
    states: HashMap<String, NFAState>,
}

#[derive(Debug)]
struct NFAState {
    is_final: bool,
    transitions: MultiMap<String, String>,
}

impl NFA {
    fn transition(&self, from_state: &str, symbol: &str) -> Option<&Vec<String>> {
        self.states
            .get(from_state)
            .unwrap()
            .transitions
            .get_vec(symbol)
    }

    pub fn resolve_transitions(&self, symbols: Vec<&str>) -> Result<Vec<String>, String> {
        let mut current_states: HashSet<&str> =
            [self.initial_state.as_ref()].iter().cloned().collect();
        for symbol in symbols {
            // Check that symbol is in alphabet
            if !self.alphabet.contains(symbol) {
                return Err(format!("Symbol {} isn't the in alphabet", symbol));
            }
            // Compute states
            let mut next_states: HashSet<&str> = HashSet::with_capacity(self.states.len());
            for current_state in current_states {
                if let Some(v) = self.transition(current_state, &symbol) {
                    for state_to_transition in v {
                        next_states.insert(state_to_transition.as_str());
                    }
                }
            }
            current_states = next_states;
        }
        Ok(current_states.iter().map(|&s| s.to_string()).collect())
    }

    pub fn is_any_state_final(&self, states: Vec<String>) -> bool {
        for state in states {
            if self.states.get(state.as_str()).unwrap().is_final {
                return true;
            }
        }
        false
    }
}

impl TryFrom<String> for NFA {
    type Error = &'static str;

    fn try_from(file: String) -> Result<Self, Self::Error> {
        let mut lines = file.lines();

        // Parse states
        let mut states: HashMap<String, NFAState> = HashMap::new();
        for state_name in lines.next().unwrap().split(',') {
            states.insert(
                state_name.to_string(),
                NFAState {
                    is_final: false,
                    transitions: MultiMap::new(),
                },
            );
        }

        // Parse alphabet
        let mut alphabet: HashSet<String> = HashSet::new();
        match lines.next() {
            Some(s) => {
                let symbols: Vec<&str> = s.split(',').collect();
                for symbol in symbols {
                    alphabet.insert(symbol.to_string());
                }
            }
            None => return Err("Couldn't find the alphabet definition"),
        };

        // Parse initial state
        let initial_state = match lines.next() {
            Some(s) => match states.contains_key(s) {
                true => s.to_string(),
                false => return Err("Couldn't find initial state"),
            },
            None => return Err("Couldn't find the initial state definition"),
        };

        // Parse final states
        let final_states: Vec<&str> = match lines.next() {
            Some(s) => s.split(',').collect(),
            None => return Err("Couldn't find the final states definition"),
        };
        for final_state in final_states {
            match states.get_mut(final_state) {
                Some(state) => state.is_final = true,
                None => return Err("Couldn't find final state"),
            }
        }

        // Parse transitions
        for line in lines {
            let mut line_iter = line.split("=>");
            let from_symbol: Vec<&str> = match line_iter.next() {
                Some(s) => s.split(',').collect(),
                None => return Err("Transition line is empty"),
            };
            let from_state = match from_symbol.get(0) {
                Some(&s) => match states.contains_key(s) {
                    true => s,
                    false => {
                        return Err("Couldn't find a transition's starting state in defined states")
                    }
                },
                None => return Err("Couldn't find starting state in a transition"),
            };
            let symbol = match from_symbol.get(1) {
                Some(&s) => match alphabet.contains(s) {
                    true => s,
                    false => return Err("Symbol doesn't belong to alphabet definition"),
                },
                None => return Err("Couldn't find symbol in transition definition"),
            };
            let next_states: Vec<&str> = match line_iter.next() {
                Some(s) => s.split(',').collect(),
                None => return Err("Transition couldn't be split correctly"),
            };
            for next_state in next_states {
                if states.contains_key(next_state) {
                    states
                        .get_mut(from_state)
                        .unwrap()
                        .transitions
                        .insert(symbol.to_string(), next_state.to_string());
                } else {
                    return Err("Couldn't find ending state in transition in defined states");
                }
            }
        }

        Ok(NFA {
            states,
            initial_state,
            alphabet,
        })
    }
}

impl fmt::Display for NFA {
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
