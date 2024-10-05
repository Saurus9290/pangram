use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool{
    sentence
    .to_lowercase()
    .chars()
    .filter(|c| c.is_ascii_alphabetic())
    .collect::<HashSet<char>>()
    .len() == 26
}
fn main(){
    let sentence1 = "The quick brown fox jumps over the lazy dog.";
    let sentence2 = "Hello, world!";

    if is_pangram(sentence1){
        println!("'{}' is a pangram.", sentence1);
    } else{
        println!("'{}' is not a pangram.", sentence1)
    }
    if is_pangram(sentence2) {
        println!("'{}' is a pangram.", sentence2);
    } else {
        println!("'{}' is not a pangram.", sentence2);
    }
}