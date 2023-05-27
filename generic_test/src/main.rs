fn main() {
    println!("Hello, world!");
}

//定义函数 generic

fn largest<T>(list:&[T]) -> &T{
    let mut largest = &list[0];
    //...
    largest
}

//定义结构体 generic

struct Point<T>{}
struct Name<T,U>{}
enum MyResult<T,E>{}

//定义方法类型
impl<T> Point<T>{}
