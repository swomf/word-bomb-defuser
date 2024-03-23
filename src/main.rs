use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use rand::Rng;
use regex::Regex;

fn read_word_list() -> (HashMap<usize, Vec<String>>, HashMap<usize, Vec<String>>) {
    let mut all_words_by_length: HashMap<usize, Vec<String>> = HashMap::new();
    let mut all_punctuated_words_by_length: HashMap<usize, Vec<String>> = HashMap::new();

    // Read file
    let file = match File::open("word-lists/combined-list.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open word-lists/combined-list.txt: {}", e);
            panic!("Quitting.");
        }
    };
    let reader = BufReader::new(file);

    // Parse file
    for line in reader.lines() {
        let word = line.unwrap();
        let length = word.len();
        if word.contains("-") || word.contains("'") {
            all_punctuated_words_by_length
                .entry(length)
                .or_insert(Vec::new())
                .push(word);
        } else {
            all_words_by_length
                .entry(length)
                .or_insert(Vec::new())
                .push(word);
        }
    }
    (all_words_by_length, all_punctuated_words_by_length)
}

struct Solver {
    all_words_by_length: HashMap<usize, Vec<String>>,
    all_punctuated_words_by_length: HashMap<usize, Vec<String>>,
    previous_words_by_length: BTreeMap<usize, Vec<String>>,
    previous_punctuated_words_by_length: BTreeMap<usize, Vec<String>>,
    previous_input: String,
}

impl Solver {
    fn new() -> Solver {
        let (all_words_by_length, all_punctuated_words_by_length) = read_word_list();

        Solver {
            all_words_by_length: all_words_by_length,
            all_punctuated_words_by_length: all_punctuated_words_by_length,
            previous_words_by_length: BTreeMap::new(),
            previous_punctuated_words_by_length: BTreeMap::new(),
            previous_input: String::new(),
        }
    }

    fn pick_words_and_print(&self, words_by_length: &BTreeMap<usize, Vec<String>>) {
        let mut print_statements = Vec::new();

        let mut rng = rand::thread_rng();
        for key in words_by_length.keys().rev() {
            let words = words_by_length.get(key).unwrap();
            let selected_word = words[rng.gen_range(0..words.len())].clone();
            let word_len_str = format!("{:02}", selected_word.len());
            print_statements.push(format!("{:02} {}", word_len_str, selected_word.clone()));
        }

        for print_statement in print_statements {
            println!("{}", print_statement);
        }
    }

    fn solve(&mut self, input: String) {
        if input == self.previous_input || input == "" {
            self.previous_input = input;
            self.pick_words_and_print(&self.previous_words_by_length);
            self.pick_words_and_print(&self.previous_punctuated_words_by_length);
            return;
        }

        let re = Regex::new(&input).unwrap();
        self.previous_input = input.clone();
        self.previous_words_by_length.clear();
        self.previous_punctuated_words_by_length.clear();

        for (length, words) in self.all_words_by_length.iter() {
            for word in words {
                if re.is_match(word) {
                    self.previous_words_by_length
                        .entry(*length)
                        .or_insert(Vec::new())
                        .push(word.clone());
                }
            }
        }

        for (length, words) in self.all_punctuated_words_by_length.iter() {
            for word in words {
                if re.is_match(word) {
                    self.previous_punctuated_words_by_length
                        .entry(*length)
                        .or_insert(Vec::new())
                        .push(word.clone());
                }
            }
        }

        self.pick_words_and_print(&self.previous_words_by_length);
        self.pick_words_and_print(&self.previous_punctuated_words_by_length);
    }
}

fn main() -> std::io::Result<()> {
    let mut word_solver = Solver::new();
    let mut user_input = String::new();
    loop {
        print!("Enter prompt: ");
        std::io::stdout().flush().unwrap();
        user_input.clear(); // Clear the string before reading new input
        std::io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_lowercase();

        word_solver.solve(user_input.clone());
    }
}
