use std::cell::Cell;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Default)]
pub struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    end: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Letter {
    Unknown,
    Known(usize),
}


fn main() {
    let mut dict = Trie::default();

    // Read dictionary file, and put all the words into a trie
    let lines = BufReader::new(File::open("normdict").unwrap()).lines();
    for line in lines {
        if let Ok(line) = line {
            trie_insert(&mut dict, line.as_str())
        }
    }

    // Read puzzle
    let mut words: Vec<String> = Vec::new();
    let lines = BufReader::new(File::open("easy.txt").unwrap()).lines();
    for line in lines {
        if let Ok(line) = line {
            for word in line.split_whitespace() {
                words.push(word.to_owned());
            }
        }
    }

    // Words now contains a list of all cipher-ed words
    // We will sort them by length - longest first
    // words.sort_by_key(|word| - (word.len() as i32));
    // println!("{:?}", words);
    
    let mut translation_table = HashMap::new();
    for c in 'A'..='Z' {
        translation_table.insert(c, Cell::new(Letter::Unknown));
    }

    let trans_words: Vec<_> = words.into_iter().map(|word| {
        word.chars().map(|c| translation_table.get(&c).unwrap()).collect::<Vec<&Cell<Letter>>>()
    }).collect();

    // Sorting the words makes it way faster as there are fewer long words than short
    let mut sorted_words = trans_words.clone();

    sorted_words.sort_by_key(|word| - (word.len() as i32));

    let used = [false; 26];

    let word_index = 0;
    let letter_index = 0;
    search(word_index, letter_index, &dict, &dict, &sorted_words, used, &trans_words);


    // recursive_trie_print(&dict, &mut String::new());
}

pub fn search(word_index: usize, letter_index: usize, dict: &Trie, cur_dict: &Trie, words: &Vec<Vec<&Cell<Letter>>>, mut used: [bool; 26], original_words: &Vec<Vec<&Cell<Letter>>>) {
    let word = match words.get(word_index) {
        Some(word) => word,
        None => {
            // println!("Success! Translation found!");
            for word in original_words {
                print_word(word)
            }
            println!();
            // print_word(words.get(0).unwrap());
            return;
        },
    };
    let letter = match word.get(letter_index) {
        Some(letter) => letter,
        None => {
            // Check whether we are at the end of a word, or just in the middle
            if cur_dict.end {
                // Completed the word! Onto next word
                return search(word_index + 1, 0, dict, dict, words, used, original_words)
            } else {
                return;
            }
        }
    };
    // print_word(word);
    match letter.get() {
        Letter::Unknown => {
            for c in 0..26_usize {
                if *used.get(c).unwrap() {
                    // The letter has already been used
                    continue;
                }
                letter.set(Letter::Known(c));
                used[c] = true;
                search(word_index, letter_index, dict, cur_dict, words, used, original_words);
                used[c] = false;
            }
            letter.set(Letter::Unknown)
        },
        Letter::Known(c) => {
            match cur_dict.nodes.get(c).unwrap() {
                // We already know the letter and it is potentially okay
                Some(new_dict) => {
                    search(word_index, letter_index + 1, dict, new_dict.as_ref(), words, used, original_words);
                },
                // The current translation is bad
                None => return,
            }
        },
    }
}

// Insert a word into the trie
pub fn trie_insert(trie: &mut Trie, word: &str) {
    let mut current = trie;

    for char in word.chars() {
        let c_val = (char as usize) - ('A' as usize);

        let temp = current.nodes.get_mut(c_val).unwrap();
        current = temp.get_or_insert(Box::new(Trie::default()))
    }

    current.end = true;
}

// Print all words in the trie. Mainly for testing that the trie works
pub fn recursive_trie_print(trie: &Trie, word: &mut String) {
    if trie.end {
        println!("{}", word);
    }
    for (c, trie) in ('A'..='Z').zip(trie.nodes.iter()) {
        match trie {
            Some(trie) => {
                word.push(c);
                recursive_trie_print(trie, word);
                word.pop();
            },
            None => (),
        }
    }
}

pub fn print_word(word: &Vec<&Cell<Letter>>) {
    let mut word_str = String::new();
    for c in word {
        word_str.push(match c.get() {
            Letter::Unknown => '?',
            Letter::Known(c) => char::from_u32((c + 'A' as usize) as u32).unwrap(),
        })
    }
    print!("{} ", word_str);
}
