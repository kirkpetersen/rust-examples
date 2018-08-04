// code to detect duplicate values in array

use std::vec::Vec;

pub fn parenthesis_match(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            match stack.pop() {
                Some('(') => (),
                _ => return false
            }
        } else {
            // ignore all other characters
        }
    }

    stack.len() == 0
}

#[test]
fn it_verifies_parenthesis() {
    assert_eq!(parenthesis_match("()"), true);
}
