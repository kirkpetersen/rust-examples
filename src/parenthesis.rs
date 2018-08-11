// code to detect duplicate values in array

use std::vec::Vec;

pub fn parenthesis_match(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == '[' {
            stack.push(c);
        } else if c == ')' {
            match stack.pop() {
                Some('(') => (),
                _ => return false
            }
        } else if c == ']' {
            match stack.pop() {
                Some('[') => (),
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
    // Examples: “(a)” => true, “(ab” => false, ")(" => false, “(a(b(c)))” => true
    // Ex: “(a[b])” => true, “([)]” => false
    // Ex: "([()))" => false
    assert_eq!(parenthesis_match(""), true);
    assert_eq!(parenthesis_match("(a)"), true);
    assert_eq!(parenthesis_match("(ab"), false);
    assert_eq!(parenthesis_match(")("), false);
    assert_eq!(parenthesis_match("(a(b(c)))"), true);
    assert_eq!(parenthesis_match("(a[b])"), true);
    assert_eq!(parenthesis_match("([)]"), false);
    assert_eq!(parenthesis_match("([()))"), false);
}
