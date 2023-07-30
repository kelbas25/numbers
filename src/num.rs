
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const INPUT_FILE: &str = "num.in";
const OUTPUT_FILE: &str = "num.out";

const ERROR_OPEN_FILE: &str = "Failed to open the file";
const ERROR_CREATE_FILE: &str = "Failed to create the file";
const ERROR_WRITE_TO_FILE: &str = "Failed to write to the file";

fn read_numbers_from_file(filename: &str) -> Vec<i64> {
    let file = File::open(filename).expect(ERROR_OPEN_FILE);
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for line in reader.lines() {
        if let Ok(number_str) = line {
            if let Ok(number) = number_str.parse::<i64>() {
                numbers.push(number);
            }
        }
    }

    numbers
}

fn write_numbers_to_file(filename: &str, numbers: &[i64]) {
    let mut output = File::create(filename).expect(ERROR_CREATE_FILE);

    for number in numbers {
        writeln!(output, "{}", number).expect(ERROR_WRITE_TO_FILE);
    }
}

fn main() {
    let mut data = read_numbers_from_file(INPUT_FILE);

    data = data.iter().map(|num| handler::replace(num)).collect();
    data = handler::merge_sort(&data);

    write_numbers_to_file(OUTPUT_FILE, &data);
}

