/*
closure 闭包:
1.匿名函数
2.可做变量，参数，返回值
3.可在其作用域内捕获值
    三种Trait：FnOnce() <- FnMut() <- Fn()
*/

fn main() {
    //创建一个closure
    let add_one = |x|{
        x+1
    };//无需返回值，参数类型，编译器自动推断，推断不了要标注

    //let add_one_v2 = |x| x+1;

    let mut c = Cacher::new(|x| x+1);

    let a = 1;
    add_one(1);
    println!("{}",c.value(a));//2



}

//使用技巧，使用结构体和闭包缓存值
struct Cacher<T>
where
    T:Fn(u32) -> u32
{
    calculation:T,
    value:Option<u32>,
}

impl<T> Cacher<T>
where
    T:Fn(u32) -> u32
{
    fn new(calculation:T) -> Cacher<T> {
        Cacher{
            calculation,
            value:None,
        }
    }

    fn value(&mut self,arg:u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}