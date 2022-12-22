use core::panic;
use std::fs::File;
// use std::io::{self, BufRead, Write, BufReader};
use itertools::Itertools;
use std::io::{self, BufRead, Write};
use std::path::Path;

struct OutChars {
    vector: Vec<char>,
}

impl OutChars {
    fn delete_reoccuring(&mut self) -> Self {
        let mut new_vec: Vec<char> = Vec::with_capacity(self.vector.len());
        for element in &self.vector {
            if new_vec.contains(element) {
                continue;
            } else {
                new_vec.push(*element);
            }
        }
        OutChars { vector: new_vec }
    }
}

fn main() {
    let mut output = 0_u64;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for mut chunk in &lines.into_iter().chunks(3) {
            let first = chunk.next().unwrap().unwrap();
            let second = chunk.next().unwrap().unwrap();
            let third = chunk.next().unwrap().unwrap();
            let mut common_chars = vec![];
            let mut out_char = OutChars {
                vector: vec![],
            };
            for letter in first.chars() {
                if second.contains(letter) {
                    common_chars.push(letter);
                }
            }
            for letter in common_chars {
                if third.contains(letter) {
                    out_char.vector.push(letter);
                }
            }
            out_char = out_char.delete_reoccuring();
            if out_char.vector.len() != 1 {
                panic!();
            }
            match out_char.vector[0] as u64 {
                65..=90 => output = output + out_char.vector[0] as u64 - 38,
                97..=122 => output = output + out_char.vector[0] as u64 - 96,
                _ => panic!(),
            }
        }
        println!("{:?}", output);
        if let Ok(mut file) = File::create("output.txt") {
            file.write(output.to_string().as_bytes()).unwrap();
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
