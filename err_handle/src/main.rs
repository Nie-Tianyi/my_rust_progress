fn main() {
    /*错误处理的4种方法
        Result 枚举 Result::Ok(T) & Result::Err(E)
        unwrap() Ok 返回Ok值，Err自动调用panic!()
        expect() Err时调用expect里面的参数
        ? 传播错误，使用：File::open("hello.txt")?; *只有函数返回值时Result时才能使用*
    */
}
