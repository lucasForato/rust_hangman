pub struct Hangman {
    hangman: String,
    word: String,
    filled: Vec<char>,
    errors: u32,
}

impl Hangman {
    pub fn new(word: String) -> Hangman {
        let mut filled: Vec<char> = Vec::new();
        for _ in 0..word.len() {
            filled.push('_');
        }

        let hangman: String =
            "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========".to_string();

        Hangman {
            hangman,
            word,
            filled,
            errors: 0,
        }
    }

    pub fn get_hangman(&self) -> &String {
        &self.hangman
    }

    pub fn get_errors(&self) -> u32 {
        self.errors
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }

    pub fn get_filled_as_word(&self) -> String {
        self.filled.iter().collect()
    }

    pub fn update_filled(&mut self, character: char, position: u32) {
        self.filled[position as usize] = character;
    }

    pub fn add_body_part(&mut self) {
        self.errors += 1;
        self.errors = std::cmp::min(self.errors, 6);

        match self.errors {
            1 => {
                println!("[ 5 ] tries left");
                self.hangman =
                    "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========".to_string();
            }
            2 => {
                println!("[ 4 ] tries left");
                self.hangman =
                    "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========".to_string()
            }
            3 => {
                println!("[ 3 ] tries left");
                self.hangman =
                    "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========".to_string()
            }
            4 => {
                println!("[ 2 ] tries left");
                self.hangman =
                    "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========".to_string()
            }
            5 => {
                println!("[ 1 ] tries left");
                self.hangman =
                    "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========".to_string()
            }
            6 => {
                println!("[ 0 ] tries left");
                self.hangman =
                    "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n=========".to_string()
            }
            _ => ()
        }
    }
}
