fn main() {
    //init a new vector
    //1. init empty vector directly
    let v:Vec<i32> = Vec::new();
    //2. use macro vec!
    let v:Vec<u32> = vec![1,2,3];

    // update a vector use .push() method
    let mut v = vec![1,2,3,4];
    v.push(5);

    println!("{:?}",v);//[1, 2, 3, 4, 5]

    //read form a vector
    //read directly
    //let third:&i32 = &v[2];
    let third:i32 = v[2];//panic if out of bound
    println!("The third element is {0}",third);
    //read vector element .get() method
    //let forth:Option<&i32> = v.get(3);
    let forth:Option<i32> = v.get(3).copied();// return None if out of bound
    match forth {
        Some(forth) => println!("The forth element is {0}",forth),
        None => println!("There is no forth element!"),
    }

    v.push(6);
    println!("{:?}",v);
    println!("{}",third);

    //iterate a vector
    let mut v = vec![100,20,50];
    for i in &v {
        println!("{i}");
    }
    for i in &mut v{
        *i += 50;
    }

    //use enum + vector store values of different type

}
