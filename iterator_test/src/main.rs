/*


*/

fn main() {
    //创建一个迭代器
    let v1 = vec![1,2,3];
    for _i in v1.iter(){//语法糖，获得所有权
        println!("{}",_i);
    }
    /*
    三种迭代方法：
        iter:元素不可变引用
        into_iter: 获得所有权
        iter_mut: 可变引用
    */

    //.next method （实现iterator trait）
    //  返回Some(v)
    //  迭代结束，返回None
    //消耗性
    //add method
    //collect method
    //非消耗性
    //map method

    let x:Vec<_> = v1.iter().map(|x| x+1).collect();
    for _i in x.iter() {
        println!("{}",_i);//2,3,4
    }

    let sum:u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a*b)
        .filter(|x|x%3==0)
        .sum();

    println!("{}",sum);
}
 struct Counter{
     counts:u32,
 }

impl Counter{
    fn new() -> Counter{
        Counter{counts:0}
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        if self.counts<5 {
            self.counts +=1;
            Some(self.counts)
        }else {
            None
        }
    }
}