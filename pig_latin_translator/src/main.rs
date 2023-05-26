use std::io;

fn main() {
    println!("input a english word:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("readline error");
    let input = input.trim();

    let input_char_array:Vec<char> = input.chars().collect();

    let vowels = ['a','e','i','o','u'];
    let mut index = 0;
    'outer:for i in input_char_array{
        for j in vowels{
            if i == j{
                break 'outer;
            }
        }
        index += 1;
    }

    if index == 0 {
        println!("translation: {}-hay",&input)
    }else {
        println!("translation: {}-{}ay",&input[index..],&input[0..index])
    }


}
