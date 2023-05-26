fn main() {
    // build a new string
    let mut s = String::new();
    let s = "init string".to_string();//implement Display trait
    let s = String::from("init string");// UTF-8 encode
    let hello = String::from("السلام عليكم");
    //append string
    let mut s = String::from("foo");
    let s2 = "bar"; //str
    s.push_str(s2);
    s.push('|');//a single character

    //use "+" concatenate strings
    let s1 = String::from("Hello,");
    let s2 = String::from("world");
    let s3 = s1 + &s2;// s1 ownership taken
    //"fn add(self,&s2) -> String {}" func signature
    //println!("{s1}");//error
    println!("{s2}");
    println!("{s3}");
    //use format! macro

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");//no ownership taken

    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s}");

    //slice String
    let hello = "Здравствуйте";
    let s = &hello[0..4];// s = "Зд"

    //iterate String
    for c in "你好世界！".chars(){
        println!("{c}");
    }

    for d in "你好世界！".bytes(){
        println!("{d}");
    }
}
