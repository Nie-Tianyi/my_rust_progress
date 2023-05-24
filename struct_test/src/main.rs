fn main() {
    let mut nty = build_user(
        String::from("1355705558@qq.com"),
        String::from("NIE Tianyi")
    );

    println!("{}",nty.email);
    nty.active = false;

    let nty2 = User{
        email:String::from("nty135570@gmail.com"),
        ..nty //这里nty：User已经失效 nty.username所有权已经给了nty2
    };

    let initial_point = Point3D(0,0,0);

    let my_subject = AlwaysEqual;

    let rect1 = Rectangle::rectangle(32,64);
    let rect2=Rectangle::square(24);

    println!("{:?}",rect1);
    println!("{}",rect1.calc_area());
    println!("{}",rect1.can_hold(&rect2));



}
struct Point3D(i64,i64,i64);

struct AlwaysEqual;

struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

fn build_user(email:String,username:String) -> User{
    User {
        active:true,
        username,//形参名称相同，语法糖
        email,
        sign_in_count:1,
    }
}
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height:i32,
}

impl Rectangle {

    fn square(size:i32) -> Self{
        Self{
            width:size,
            height:size,
        }
    }

    fn rectangle(width:i32,height:i32) -> Self {
        Self {
            width,
            height,
        }
    }

    fn calc_area(&self) -> i32{
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle)-> bool {
        self.width > other.width && self.height > other.height
    }

}

