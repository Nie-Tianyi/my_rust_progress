use std::io;
use std::option::Option;

fn main() {
    let string = String::new();
    let mut add_one = Cacher::new(|x| x+1);
    let x1 = add_one.value(123);
    println!("{}",x1);
}

struct Cacher<T>
where
    T:Fn(i32) -> i32
{
    calculate:T,
    value:None,
}

impl<T> Cacher<T>
where
    T:Fn(i32) -> i32
{
    fn new(calc:T) -> Cacher<T>{
        Cacher{
            calculate : T,
            value:None
        }
    }

    fn value(&mut self,level:i32) -> Some(i32){
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculate)(level);
                self.value = v;
                v
            }
        }
    }

}