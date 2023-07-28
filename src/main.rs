use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::vec::Vec;

const INPUT_FILE: &str = "num.in";
const OUTPUT_FILE: &str = "num.out";
const ERROR_OPEN_FILE: &str = "Failed to open the file";
const ERROR_CREATE_FILE: &str = "Failed to create the file";
const ERROR_WRITE_TO_FILE: &str = "Failed to write to the file";

fn main() {
    let mut data = read_numbers_from_file(INPUT_FILE);

    data = data.iter().map(|num| replace(num)).collect();
    data = merge_sort(&data);

    write_numbers_to_file(OUTPUT_FILE, &data);
}

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

fn replace(number: &i64) -> i64 {
    let mut number_str = number.to_string();
    number_str = number_str.replace("123", "321");

    number_str.parse().unwrap_or(0)
}

fn merge_sort(vector: &Vec<i64>) -> Vec<i64>{
    if vector.len() < 2 {
       return vector.to_vec()
    }

    let left = merge_sort(&vector[0..vector.len()/2].to_vec());
    let right = merge_sort(&vector[vector.len()/2..].to_vec());

    let mut left_iter = 0;
    let mut right_iter = 0;
    let mut merged: Vec<i64> = Vec::new();

    while left_iter < left.len() && right_iter < right.len() {
        if left[left_iter] < right[right_iter]{
            merged.push(left[left_iter]);
            left_iter += 1;
        }else{
            merged.push(right[right_iter]);
            right_iter += 1;
        }
    }

    while left_iter < left.len(){
        merged.push(left[left_iter]);
        left_iter += 1;
    }

    while right_iter < right.len(){
        merged.push(right[right_iter]);
        right_iter += 1;
    }

    merged
}

fn write_numbers_to_file(filename: &str, numbers: &[i64]) {
    let mut output = File::create(filename).expect(ERROR_CREATE_FILE);

    for number in numbers {
        writeln!(output, "{}", number).expect(ERROR_WRITE_TO_FILE);
    }

}
