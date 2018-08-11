# rust-examples

Just trying out some Rust. Trying to get comfortable with it.

Stuff I want to do:

* Get comfortable with the ownership model
* Get comfortable with types, structs, traits, etc.
* Try writing something "real", to get a feel for performance

# Trie example

This is an experiment with data structures and an attempt to implement
a Trie for dealing with things like IP routes.

* enum? struct? both?
* always have a root node with 0.0.0.0/0 (or ::/0)?
* trait for the key stored in the trie? Something like "Cidr",
  with the ability to iterate over the bits, as well as construct new Cidr
  objects based on the parent + left|right
* value?
