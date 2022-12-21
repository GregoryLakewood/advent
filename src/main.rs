use core::panic;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    let mut output = 0_u64;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let (left, right) = ip.split_at(ip.len() / 2);
                for letter in left.chars() {
                    if right.contains(letter) {
                        let mut buffer: [u8; 1] = [0; 1];
                        letter.encode_utf8(&mut buffer);
                        match buffer[0] as u64 {
                            65..=90 => output = output + buffer[0] as u64 - 38,
                            97..=122 => output = output + buffer[0] as u64 - 96,
                            _ => panic!(),
                        }
                        break;
                    }
                }
            } else {
                println!("someting went bad: {:?}", line);
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
