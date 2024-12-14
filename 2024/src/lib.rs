pub mod util {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Read};

    pub fn get_reader(day: &str, is_sample: bool) -> Result<BufReader<File>, io::Error> {
        let file = File::open(format!(
            "src/bin/{}/{}input.txt",
            day,
            if is_sample { "sample_" } else { "" }
        ))?;
        Ok(BufReader::new(file))
    }

    pub fn read_input(day: &str, is_sample: bool) -> Result<Vec<Vec<i32>>, io::Error> {
        let reader = get_reader(day, is_sample)?;
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

    pub fn read_full_input(day: &str, is_sample: bool) -> Result<String, io::Error> {
        let mut reader = get_reader(day, is_sample)?;
        let mut result = String::new();
        reader.read_to_string(&mut result)?;
        Ok(result)
    }

    pub fn read_char_input(day: &str, is_sample: bool) -> Result<Vec<String>, io::Error> {
        let reader = get_reader(day, is_sample)?;
        let mut result = vec![];
        for line in reader.lines() {
            result.push(line?);
        }
        Ok(result)
    }

    pub fn read_board(day: &str, is_sample: bool) -> Result<Vec<Vec<String>>, io::Error> {
        let reader = get_reader(day, is_sample)?;
        let mut result = vec![];
        for line in reader.lines() {
            result.push(line?.chars().map(|c| c.to_string()).collect());
        }
        Ok(result)
    }
}
