
/**
crate 是 rust 最小代码单位
    包含 二进制项（一个服务器软件）(有main主函数入口)
        库（一个lib）（无主函数入口）
crate root 编译器起点

package 包含一个或者多个crate

src/bin/ 目录下可以有多个二进制文件，每个都可以被编译为一个独立的crate

 */



fn main() {
    println!("Hello, world!");
}
