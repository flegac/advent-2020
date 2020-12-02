use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn parse_file(filename: &str) -> Vec<i32> {
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    let mut all = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                all.push(value.parse::<i32>().unwrap());
            }
        }
    }
    // all.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return all;
}
