// Run the example code from here

mod duplicates;
mod parenthesis;

use duplicates::*;
use parenthesis::*;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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
    assert_eq!(parenthesis_match(")("), false);

    // Try out ipv4/ipv6 addresses and cidr ranges
    let v4: Ipv4Addr = "1.2.3.4".parse().unwrap();
    println!("v4: {}", v4);
    let v6: Ipv6Addr = "2001::1".parse().unwrap();
    println!("v6: {}", v6);

    let x = "x1.2.3.4".parse::<IpAddr>();

    match x {
        Ok(addr) => println!("addr: {}", addr),
        Err(e) => println!("could't parse! {}", e)
    }
}
