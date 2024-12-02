pub mod util {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Read};

    pub fn read_input(day: &str) -> Result<Vec<Vec<i32>>, io::Error> {
        let file = File::open(format!("src/bin/{}/input.txt", day))?;
        let reader = BufReader::new(file);
        let mut result = vec![];
        for line in reader.lines() {
            result.push(
                line?
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
        Ok(result)
    }

    pub fn read_full_input(day: &str) -> Result<String, io::Error> {
        let file = File::open(format!("src/bin/{}/input.txt", day))?;
        let mut reader = BufReader::new(file);
        let mut result = String::new();
        reader.read_to_string(&mut result)?;
        Ok(result)
    }
}
