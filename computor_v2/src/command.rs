use std::collections::VecDeque;


const MAX_COMMAND: usize = 1000;


#[derive(Debug, PartialEq)]
pub struct Commands {
    data: VecDeque<Box<(String, String)>>,
}


impl Commands {
    pub fn new() -> Commands {
        Commands { data: VecDeque::new() }
    }

    pub fn push(&mut self, command: String, result: String) {
        self.data.push_back(Box::new((command.trim().to_string(), result.trim().to_string())));
        while self.data.len() > MAX_COMMAND {
            self.data.pop_front();
        }
    }

    pub fn show(&self) -> String {
        let mut string = String::new();
        for b in &self.data {
            string += format!("  {}: {}\n", b.0, b.1).as_str();
        }
        string
    }

    pub fn at(&self, index: usize) -> Option<String> {
        if self.data.len() <= index {
            None
        } else {
            match self.data.get(self.data.len() - index - 1) {
                Some(b) => Some(b.0.clone()),
                None => None,
            }
        }
    }
}
