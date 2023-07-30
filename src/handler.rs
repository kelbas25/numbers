use std::vec::Vec;

pub fn replace(number: &i64) -> i64 {
    let mut number_str = number.to_string();
    number_str = number_str.replace("123", "321");

    number_str.parse().unwrap_or(0)
}

pub fn merge_sort(vector: &Vec<i64>) -> Vec<i64> {
    if vector.len() < 2 {
        return vector.to_vec()
    }

    let left = merge_sort(&vector[0..vector.len() / 2].to_vec());
    let right = merge_sort(&vector[vector.len() / 2..].to_vec());

    let mut left_iter = 0;
    let mut right_iter = 0;
    let mut merged: Vec<i64> = Vec::new();

    while left_iter < left.len() && right_iter < right.len() {
        if left[left_iter] < right[right_iter] {
            merged.push(left[left_iter]);
            left_iter += 1;
        } else {
            merged.push(right[right_iter]);
            right_iter += 1;
        }
    }

    while left_iter < left.len() {
        merged.push(left[left_iter]);
        left_iter += 1;
    }

    while right_iter < right.len() {
        merged.push(right[right_iter]);
        right_iter += 1;
    }

    merged
}


