extern crate rand;

use rand::{Rng, thread_rng};
use std::io::BufRead;
use std::path::Path;
use std::fs::File;
use std::io;

struct Word {
    answer: String,
    length: usize,
    correct_count: usize,
    representation: String
}

trait CheckLetter {
    fn check_for_letter(&mut self, c: char) -> bool;
}

trait CheckWord {
    fn check_for_word(&mut self, inp_word: String) -> bool;
}

trait CheckComplete {
    fn check_complete(&self) -> bool;
}

impl CheckComplete for Word {
    fn check_complete(&self) -> bool {
        self.correct_count == self.length
    }
}

impl CheckWord for Word {
    fn check_for_word(&mut self, inp_word: String) -> bool {
        if inp_word.eq(&self.answer) {
            true
        } else {
            false
        }
    }
}

impl CheckLetter for Word {
    fn check_for_letter(&mut self, c: char) -> bool {
        let mut count = 0;
        let mut representation_resp = String::with_capacity(self.length);
        let mut found = false;
        let mut index_resp = 0;
        for l in self.answer.chars() {
            if l == c {
                found = true;
                count+=1;
                representation_resp.push(l);
            } else {
                if self.representation.chars().nth(index_resp) != Some('_') {
                    representation_resp.push(self.representation.chars().nth(index_resp).unwrap());
                } else {
                    representation_resp.push('_');
                }
            }
            index_resp += 1;
        }
        if found == true {
            self.correct_count += count;
            println!("Found {}", c);
        }
        self.representation = representation_resp;
        found
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_list_of_words(filename: String) -> Vec<String> {
    let mut w = Vec::<String>::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let word = line.unwrap();
            w.push(word);
        }
    }
    w
}

fn select_word(filename: String) -> String{
    let mut rn = thread_rng();
    let words = read_list_of_words(filename);
    let word_count = words.len();
    let random_select = rn.gen_range(0..word_count);
    let selected_word = words.get(random_select).unwrap();
    selected_word.to_string()
}

fn main() {
    let body = vec!["head".to_string(), "neck".to_string(), "torso".to_string(), "left_hand".to_string(), 
    "right_hand".to_string(), "left_leg".to_string(), "right_leg".to_string(), "left_foot".to_string(), "right_foot".to_string()];
    let mut body_iter = body.iter();
    let result = select_word("./words.txt".to_string());
    let mut select_word = Word {
        length: result.len(),
        representation: String::from_utf8(vec!(b'_'; result.len())).unwrap(),
        answer: result,
        correct_count: 0
    };
    let mut input_ch: char;
    let mut body_comp = false;
    while !select_word.check_complete() && !body_comp {
        println!("Pls enter a char or complete word");
        let mut input_str = String::new();
        match io::stdin().read_line(&mut input_str) {
            Ok(_) => {
                if input_str.trim().len() > 1 {
                    if select_word.check_for_word(input_str.trim().to_string()) {
                        select_word.representation = String::from_iter(select_word.answer.chars());
                        select_word.correct_count = select_word.length;
                    } else {
                        let next_part = body_iter.next().unwrap();
                        println!("Wrong answer, next part is {}", next_part);
                        if next_part == "right_foot" {
                            body_comp = true;
                        }
                    }
                } else {
                    input_ch = input_str.chars().nth(0).unwrap();
                    if select_word.check_for_letter(input_ch) {
                        println!("Found the letter {}, word now is {}", input_ch, select_word.representation);
                    } else {
                        let next_part = body_iter.next().unwrap();
                        println!("Wrong answer, next part is {}", next_part);
                        if next_part == "right_foot" {
                            body_comp = true;
                        }
                    }
                }
            }
            Err(_) => {
                println!("Invalid or no input");
            }
        }
    }
    if body_comp {
        println!("You were unsuccessful in guessin the word: {}", select_word.answer);
    } else {
        println!("Congratz, you are successful with answer: {}", select_word.answer);
    }
}
