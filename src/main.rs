use std::fs::{File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    let mut output = vec![0, 0, 0_u64];
    let mut counter: u64 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    for value in output.iter().enumerate() {
                        if counter > *value.1 {
                            output.insert(value.0, counter);
                            output.pop();
                            break;
                        }
                    }
                    counter = 0;
                    continue;
                }
                counter = counter + ip.parse::<u64>().unwrap();
            } else {
                println!("someting went bad: {:?}", line);
            }
        }
        println!("{:?}", output);
        println!("{}", output.iter().sum::<u64>());
        if let Ok(mut file) = File::create("output.txt") {
            file.write(output.iter().sum::<u64>().to_string().as_bytes())
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
