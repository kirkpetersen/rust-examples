#[derive(Debug)]
pub struct TrieNode<K, V> {
    key: K,
    value: V,
    l: Option<Box<TrieNode<K, V>>>,
    r: Option<Box<TrieNode<K, V>>>
}

impl <K, V> TrieNode<K, V> {
    pub fn new(k: K, v: V) -> TrieNode<K, V> {
        TrieNode { key: k, value: v, l: None, r: None }
    }
}

#[test]
fn check_the_trie_stuff() {
    let t = TrieNode {
        key: 0x0a010101,
        value: 0x01,
        l: None,
        r: None
    };

    match t {
        TrieNode { value: 0x01, key: x, .. } => println!("value: {}", x),
        _ => println!("not sure?")
    }

    println!("t: {:#?}", t);
}
