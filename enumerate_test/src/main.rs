use std::io;


fn main() {

    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    impl IpAddr{
        //枚举的方法有什么用？？？
    }

    let home = IpAddr::V4(127,0,0,1);

    let loopback = IpAddr::V6(String::from("::1"));

    //let some_number:i8 = Some(5);//error
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; //编译器推断不了，要指定类型

    let another_number:i8 = 8;
    //let sum = another_number + some_number;//error
    #[derive(Debug)]
    enum UsState{
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin:Coin) -> i32{
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) =>{
                println!("State Quarter is from {:?}", state);
                25
            }//match必须被穷尽
        }
    }

    let dice_roll = 3;

    match dice_roll {
        3 => println!("You got 3"),
        a=> println!("You got a {}",a),
    }

    //if..let..else.. 语法糖 只匹配一种情况
    if let 3 = dice_roll {
        println!("You got 3");
    }else {
        println!{"You Got {}",dice_roll};
    }





    value_in_cents(Coin::Quarter(UsState::Alaska));

}




fn enum_test(){
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr{
        kind: IpAddrKind,
        address: String,
    }
    impl IpAddr {
        fn ip_address(kind:IpAddrKind, address:String) -> Self{
            Self{
                kind,
                address,
            }
        }


    }

    println!("{:?}",IpAddrKind::V4); //V4

    let home = IpAddr::ip_address(IpAddrKind::V4,String::from("127.0.0.1"));

    let loopback = IpAddr::ip_address(IpAddrKind::V6,String::from("::1"));

    println!("{:?}",home);//IpAddr { kind: V4, address: "127.0.0.1" }
    println!("{:?}",loopback);//IpAddr { kind: V6, address: "::1" }
}





