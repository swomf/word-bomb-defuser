use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use rand::Rng;
use regex::Regex;

/// Only used once in `Solver::new` to collate all word-lists, then sort
/// by punctuation `' -` and non-punctuation
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
                    let word = line.unwrap().trim().to_lowercase();
                    if word.len() != 0 {
                        all_words_set.insert(word.trim().to_lowercase());
                    }
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

pub struct Solver {
    /// A map of lengths to lists of words of that length. Not meant to be changed.
    all_words_by_length: HashMap<usize, Vec<String>>,
    /// A map of lengths to list of punctuated words of that length. Not meant to be changed.
    all_punctuated_words_by_length: HashMap<usize, Vec<String>>,
    /// Solutions of the latest query.
    solution_words_by_length: HashMap<usize, Vec<String>>,
    /// Punctuated solutions of the latest query. Punctuation includes ' and -
    solution_punctuated_words_by_length: HashMap<usize, Vec<String>>,
    previous_input: String,
}

impl Solver {
    pub fn new() -> Solver {
        let (all_words_by_length, all_punctuated_words_by_length) = init_word_lists();

        Solver {
            all_words_by_length: all_words_by_length,
            all_punctuated_words_by_length: all_punctuated_words_by_length,
            solution_words_by_length: HashMap::new(),
            solution_punctuated_words_by_length: HashMap::new(),
            previous_input: String::new(),
        }
    }

    fn build_solution_list(
        source_list: &HashMap<usize, Vec<String>>,
        regex: &Regex,
        list_to_overwrite: &mut HashMap<usize, Vec<String>>,
    ) {
        list_to_overwrite.clear();
        for (length, words) in source_list.iter() {
            for word in words {
                if regex.is_match(word) {
                    list_to_overwrite
                        .entry(length.clone())
                        .or_insert(Vec::new())
                        .push(word.clone());
                }
            }
        }
    }

    fn format_solution_list(solution_list: &mut HashMap<usize, Vec<String>>) -> Vec<String> {
        let mut solution_list_formatted = Vec::new();
        for (length, words) in solution_list.iter_mut() {
            if words.len() == 0 {
                continue;
            }
            let random_index = rand::thread_rng().gen_range(0..words.len());
            let selected_word = words.remove(random_index);
            let word_len_str = format!("{:02}", length);
            solution_list_formatted.push(format!("{:02} {}", word_len_str, selected_word.clone()));
        }

        solution_list_formatted.sort();
        solution_list_formatted
    }

    pub fn solve_prompt(&mut self, input: String) -> (usize, Vec<String>, Vec<String>) {
        if input != self.previous_input && !input.is_empty() {
            // Input is new, do not reuse previous solution list
            let re = Regex::new(&input).unwrap();
            self.previous_input = input.clone();

            // Build solution lists
            Solver::build_solution_list(
                &self.all_words_by_length,
                &re,
                &mut self.solution_words_by_length,
            );
            Solver::build_solution_list(
                &self.all_punctuated_words_by_length,
                &re,
                &mut self.solution_punctuated_words_by_length,
            );
        }

        // Format and return the solution lists
        (
            self.get_solution_set_size(), // call before changing solution_words
            Solver::format_solution_list(&mut self.solution_words_by_length),
            Solver::format_solution_list(&mut self.solution_punctuated_words_by_length),
        )
    }

    pub fn get_size(&self) -> usize {
        let mut total_words = 0;
        for words in self.all_words_by_length.values() {
            total_words += words.len();
        }
        for words in self.all_punctuated_words_by_length.values() {
            total_words += words.len();
        }

        total_words
    }

    fn get_solution_set_size(&self) -> usize {
        let mut total_solution_words = 0;
        for words in self.solution_words_by_length.values() {
            total_solution_words += words.len();
        }
        for words in self.solution_punctuated_words_by_length.values() {
            total_solution_words += words.len();
        }

        total_solution_words
    }
}
