use std::fs::{File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    let mut output = 0_u64;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                match ip.as_str() {
                    "A X" => output += 4,
                    "B X" => output += 1,
                    "C X" => output += 7,
                    "A Y" => output += 8,
                    "B Y" => output += 5,
                    "C Y" => output += 2,
                    "A Z" => output += 3,
                    "B Z" => output += 9,
                    "C Z" => output += 6,
                    "" => break,
                    _ => panic!(),
                }

            } else {
                println!("someting went bad: {:?}", line);
            }
        }
        println!("{:?}", output);
        if let Ok(mut file) = File::create("output.txt") {
            file.write(output.to_string().as_bytes())
                .unwrap();
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
