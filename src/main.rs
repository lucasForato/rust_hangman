mod hangman;
use hangman::Hangman;
use random_word::Lang;
use std::io;

fn main() {
    let random_word = random_word::gen_len(4, Lang::En).expect("Failed to generate word").to_string();
    let mut hangman: Hangman = Hangman::new(random_word);

    loop {
        print_hangman(&hangman);
        let res: u32 = choose_path().expect("Failed to read input");

        if res == 1 {
            match guess_letter(&mut hangman) {
                true => println!("you found a correct letter"),
                false => hangman.add_body_part(),
            }

            if hangman.get_errors() == 6 {
                print_final_hangman(&hangman);
                println!("You have lost!");
                break;
            }
        }

        if res == 2 {
            match guess_word(&mut hangman) {
                true => println!("You guessed the word!"),
                false => {
                    print_final_hangman(&hangman);
                    println!("You have lost!");
                    break;
                }
            };
        }
    }
}

fn print_final_hangman(hangman: &Hangman) {
    println!("");
    println!("{}", hangman.get_hangman());
    println!("The word was: {}", hangman.get_word().to_string());
    println!("");
}

fn print_hangman(hangman: &Hangman) {
    println!("");
    println!("{}", hangman.get_hangman());
    println!("The word is: {}", hangman.get_filled_as_word());
    println!("");
}

fn guess_letter(mut hangman: &mut Hangman) -> bool {
    println!("");
    println!("Guess a letter: ");

    let mut input = String::new();
    let letter: char = match io::stdin().read_line(&mut input) {
        Ok(_) => match input.replace("\n", "").len() > 1 {
            true => {
                println!("Please enter only one letter");
                guess_letter(&mut hangman);
                return false;
            }
            false => input.chars().next().unwrap(),
        },
        Err(_) => ' ',
    };
    println!("{}", letter);

    let word = hangman.get_word().clone();

    let mut found_letter = false;
    for (i, hangman_char) in word.chars().enumerate() {
        if letter == hangman_char {
            let index = i as u32;
            found_letter = true;
            hangman.update_filled(letter, index);
        }
    }

    found_letter
}

fn guess_word(hangman: &mut Hangman) -> bool {
    println!("");
    println!("Guess the word: ");

    let mut input = String::new();
    let word: String = match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => "".to_string(),
    };

    match word.eq(hangman.get_word()) {
        true => true,
        false => false,
    }
}

fn choose_path() -> Result<u32, io::Error> {
    println!(
        "
Do you want to guess a letter or the word?
1. Guess a letter
2. Guess the word
"
    );

    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            if user_input.trim() == "1" {
                Ok(1)
            } else if user_input.trim() == "2" {
                Ok(2)
            } else {
                println!("Invalid input, please try again.");
                choose_path()
            }
        }
        Err(error) => Err(error),
    }
}
