// Run the example code from here

mod duplicates;
mod parenthesis;

use duplicates::*;
use parenthesis::*;

fn main() {
    // Find duplicates in an array
    println!("duplicates:");

    let v = vec![1,2,2];

    if contains_duplicates(&v) {
        println!("duplicates as expected!");
    } else {
        println!("no duplicates?");
    }

    println!("first duplicate: {}", find_first_duplicate(&v));

    // Check to see if parenthesis match
    println!("parenthesis:");

    if parenthesis_match("()") {
        println!("parenthesis match as expected!")
    } else {
        println!("parenthesis do not match?")
    }

    assert_eq!(parenthesis_match("("), false);
    assert_eq!(parenthesis_match(")("), false)
}
