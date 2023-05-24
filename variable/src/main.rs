use std::io;

fn main(){

    print_fibonacci_sequence(50)
}

fn temperature_converter(){
    //华氏度＝32＋摄氏度×1.8
    //摄氏度＝（华氏度－32）÷1.8

    println!("转换华氏度和摄氏度：");
    loop {
        println!("1.华氏度转换摄氏度 ( F2C ) \n2.摄氏度转换华氏度 ( C2F )\n3.exit ( exit )");

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("read line cmd failure");

        let cmd = cmd.trim();

        println!("input a number:");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("read line num failure");

        let num:f64 = num
            .trim()
            .parse()
            .expect("parse num error");


        match &cmd as &str {
            "F2C" => println!("转换为摄氏度为 {0} C°", (num - 32.0)/1.8),
            "C2F" => println!("转换为华氏度为 {0} F°", num * 1.8 + 32.0),
            "exit" => break,
            _=>(),
        }
    }
}


fn print_fibonacci_sequence(n:u64){
    let mut a:u64 = 1;
    let mut b:u64 = 1;
    println!("{}",a);
    println!{"{}",b};
    for i in 1..n {
        println!("{}",a + b);
        if i % 2 == 1{
            a = a + b;
        }else{
            b = a + b;
        }
    }

}

