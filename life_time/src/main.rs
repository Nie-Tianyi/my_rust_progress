/*
编译器推断变量生命周期三大规则
1.  编译器为每一个引用参数都分配一个生命周期参数
2.  规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
3.  如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
    说明是个对象的方法 ，那么所有输出生命周期参数被赋予 self 的生命周期

所有编译器推断不出来的生命周期都要自己手动标注，不然编译器不给过

4.* 特别地，所有的字符串字面值都拥有 'static 生命周期，拥有‘static生命周期能够存活于真个程序期间

*/

fn main() {
    let nty = User{
        username : String::from("nty"),
        email : String::from("135570@gmail.com"),
        sign_in_count : 1,
        active : true
    };

    let nty2 = User{
        username : String::from("nty1355"),
        email : String::from("135570@gmail.com"),
        sign_in_count : 1,
        active : true
    };
    let user = longest_user(&nty, &nty2);
}


fn longest<'a>(x:&'a str,y:&'a str) -> &'a str{
    if x.len() > y.len() {
        return x;
    }
    return y;
}

fn longest_user<'a>(x:&'a User,y:&'a User) -> &'a User{
    if x.username.len() > y.username.len() {
        return x;
    }
    return y;
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn active(&self){
        self.active = true;
    }
}
