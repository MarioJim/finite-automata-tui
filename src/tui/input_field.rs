use tuikit::key::Key;
use tuikit::term::Term;

#[derive(Debug)]
pub struct InputField(String);

impl InputField {
    pub fn new() -> Self {
        InputField(String::new())
    }

    pub fn receive_event(&mut self, key: Key) {
        match key {
            Key::Char(ch) => {
                if (ch >= 'a' && ch <= 'z') || ch == ' ' {
                    self.0.push(ch);
                }
            }
            Key::Backspace => {
                self.0.pop();
            }
            _ => (),
        };
    }

    pub fn print_in_term(&self, term: &Term, row: usize) {
        let _ = term.print(row, 0, self.0.as_str());
        let _ = term.set_cursor(row, self.0.len());
    }

    pub fn get_value(&self) -> &str {
        self.0.as_str()
    }

    pub fn get_split_value(&self) -> Vec<&str> {
        self.get_value().split_whitespace().collect()
    }
}
