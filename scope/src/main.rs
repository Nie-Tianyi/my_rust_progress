fn main() {

}

fn reference_test(){
    let _b = 1;
    let a = 2; //variable in stack
    if a == 2 {
        let c = 3;
        println!("{}",c);
    }

    //println!("{}",c);
    //error, cannot find variable c in this scope
    let mut s = String::from("Hello"); // new variable in heap
    s.push_str(", world");
    println!("{}",s);
    //所有权
    //引用
    //可变引用：一个变量只能有一个可变引用 或者 多个引用
    //垂悬引用 dangling reference 不允许 （空指针）
}
fn first_word(s:&str) -> &str {// &String 自动转换成 &str (= &String[..])
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
