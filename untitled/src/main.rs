
fn main() {
    let y = 2;
    let mut add_one = Cacher::new(|x| x* y);
    let x1 = add_one.value(123);
    println!("{}",x1);
}

struct Cacher<T>
where
    T:Fn(i32) -> i32
{
    calculate:T,
    value:Option<i32>,
}

impl<T> Cacher<T>
where
    T:Fn(i32) -> i32
{
    fn new(calc:T) -> Cacher<T>{
        Cacher{
            calculate : calc,
            value:None,
        }
    }

    fn value(&mut self,level:i32) -> i32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculate)(level);
                self.value = Some(v);
                v
            }
        }
    }

}