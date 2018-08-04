// code to detect duplicate values in array

use std::collections::HashSet;

pub fn contains_duplicates(array : &Vec<i32>) -> bool {
    let mut duplicates = HashSet::new();

    for value in array {
        if duplicates.contains(value) {
            return true;
        } else {
            duplicates.insert(value);
        }
    }

    false
}

pub fn find_first_duplicate(array : &Vec<i32>) -> i32 {
    let mut duplicates = HashSet::new();

    for value in array {
        if duplicates.contains(value) {
            return *value;
        } else {
            duplicates.insert(value);
        }
    }

    -1
}

#[test]
fn it_detects_duplicates() {
    assert_eq!(contains_duplicates(&vec![]), false);
    assert_eq!(contains_duplicates(&vec![1]), false);
    assert_eq!(contains_duplicates(&vec![3, 1, 2, 3]), true);
}

#[test]
fn returns_duplicates() {
    assert_eq!(find_first_duplicate(&vec![]), -1);
    assert_eq!(find_first_duplicate(&vec![3,1,2,3]), 3);
}
