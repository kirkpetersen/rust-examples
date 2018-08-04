// Run the example code from here

mod duplicates;
mod parenthesis;

use duplicates::*;
use parenthesis::*;

fn main() {
    println!("duplicates:");

    let v = vec![1,2,2];

    if contains_duplicates(&v) {
        println!("duplicates as expected!");
    } else {
        println!("no duplicates?");
    }

    println!("first duplicate: {}", find_first_duplicate(&v));

    println!("parenthesis:");

    if parenthesis_match("()") {
        println!("parenthesis match as expected!")
    } else {
        println!("parenthesis do not match?")
    }
}
