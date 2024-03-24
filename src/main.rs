use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use rand::Rng;
use regex::Regex;

fn init_word_lists() -> (HashMap<usize, Vec<String>>, HashMap<usize, Vec<String>>) {
    let mut all_words_by_length: HashMap<usize, Vec<String>> = HashMap::new();
    let mut all_punctuated_words_by_length: HashMap<usize, Vec<String>> = HashMap::new();

    // Read and parse each file in word-lists/component-lists/*.txt into
    // a set of unique words
    let mut all_words_set = HashSet::new();
    let folder = "word-lists";
    let paths = std::fs::read_dir(folder).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() && path.extension().unwrap() == "txt" {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".txt") {
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let word = line.unwrap();
                    all_words_set.insert(word.trim().to_lowercase());
                }
            }
        }
    }

    // Place words in a list of unpunctuated and a list of punctuated words
    for word in all_words_set {
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
                .push(word.clone());
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
        let (all_words_by_length, all_punctuated_words_by_length) = init_word_lists();

        Solver {
            all_words_by_length: all_words_by_length,
            all_punctuated_words_by_length: all_punctuated_words_by_length,
            previous_words_by_length: BTreeMap::new(),
            previous_punctuated_words_by_length: BTreeMap::new(),
            previous_input: String::new(),
        }
    }

    fn pick_words_and_print(&mut self) {
        // clear terminal screen
        print!("\x1B[2J\x1B[1;1H");
        let mut print_statements = Vec::new();
        let mut rng = rand::thread_rng();
        for words_by_length in [&mut self.previous_words_by_length, &mut self.previous_punctuated_words_by_length] {
            let keys: Vec<_> = words_by_length.keys().rev().cloned().collect();
            for key in keys {
                if let Some(words) = words_by_length.get_mut(&key) {
                    let words_len = words.len();
                    if words_len == 0 {
                        continue;
                    }
                    let desired_word_index = rng.gen_range(0..words_len);
                    let selected_word = words.remove(desired_word_index);
                    let word_len_str = format!("{:02}", selected_word.len());
                    print_statements.push(format!("{:02} {}", word_len_str, selected_word.clone()));
                }
            }
        }

        for print_statement in print_statements {
            println!("{}", print_statement);
        }
    }

    fn solve(&mut self, input: String) {
        if input == self.previous_input || input == "" {
            self.previous_input = input;
            self.pick_words_and_print();
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

        self.pick_words_and_print();
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
