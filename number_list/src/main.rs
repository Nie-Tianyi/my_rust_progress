use std::io;
use std::collections::HashMap;

fn main() {
    let mut number_list:Vec<i32> = Vec::new();
    //get user input
    loop{
        println!("input a number (input \"confirm\" to exit):");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("read line error");

        if let "confirm" = number.trim(){
            break;
        }

        let number:i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("input a NUMBER!!!");
                continue;
            },
        };
        number_list.push(number);
    }

    println!("{:?}",number_list);

    //get the Median and Mode of the number list

    let n = number_list.len();

    //bubble sort
    for i in 0..n{
        let mut flag:bool = true;
        for j in 0..n-1-i{
            if number_list[j] > number_list[j + 1] {

                // let tmp:i32 = number_list[j];
                // number_list[j] = number_list[j+1];
                // number_list[j+1] = tmp;
                number_list.swap(j,j+1);

                flag = false
            }
        }
        if flag {
            break;
        }
    }

    //println!("{:?}",number_list);

    //print the median

    if n%2 == 1{
        println!("The median of your input is {}",number_list.get(n/2).unwrap());
    }else {
        println!("The median of your input is {}",{
            (number_list.get(n/2 - 1).unwrap() + number_list.get(n/2).unwrap()) as f64 / 2.0
        })
    }

    //print the mode

    let mut hashmap = HashMap::new();

    for num in &number_list{
        let count = hashmap.entry(num).or_insert(0);
        *count += 1;
    }

    //println!("{:?}",hashmap);
    let mut most_frequency = 0;
    for num in &number_list{
        let i = hashmap.get(&num).copied().unwrap();
        if i > most_frequency{
            most_frequency = i;
        }
    }

    //println!{"{most_frequency}"};
    let mut mod_numbers = HashMap::new();

    for num in &number_list{
        if hashmap.get(&num).copied().unwrap() == most_frequency && most_frequency > 1{
            mod_numbers.insert(*num,0);
        }
    }

    print!("The mode number of you input is :");

    for (key,_value) in &mod_numbers{
        print!("{},",key);
    }
    println!("");

}

