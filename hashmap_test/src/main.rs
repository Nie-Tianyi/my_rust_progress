use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("John"),100);
    scores.insert(String::from("Bob"),99);//String 一旦插入 hashmap， 所有权归hashmap

    scores.insert(String::from("Bob"),97);//原有的Bob值会被替换

    scores.entry(String::from("John")).or_insert(100);//检查John有没有值，没有的话插入100


    let score_bob = scores.get("Bob").copied().unwrap_or(-1);
    println!("{score_bob}");
    //iterate
    for (key,value) in &scores{
        println!("{key},{value}");
    }

    let mut index_book = HashMap::new();

    index_book.insert("Rust Programming","Shelf 4 level 1");
    let location = index_book.get("Rust Programming").copied().unwrap_or("None");
    println!("{location}");

    let text = "hello world wonderful world";
    let counter = words_counter(text);
    println!("{:?}",counter);
}

fn words_counter(text:&str) -> HashMap<&str,i32>{
    let mut result = HashMap::new();

    for word in text.split_whitespace() {
        let count = result.entry(word).or_insert(0);
        //.entry return Entry:enum
        //.or_insert return mutable reference of a value
        *count += 1;
    }

    result

}
