use std::collections::HashMap;
use std::io;

fn main() {
    let mut database:HashMap<String,String> = HashMap::new();
    let mut names_table:Vec<String> = Vec::new();
    loop{
        let mut user_cmd = String::new();
        io::stdin().read_line(&mut user_cmd).expect("readline error");

        let user_cmd = user_cmd.trim();

        let user_cmd:Vec<&str> = user_cmd.split_whitespace().collect();
        println!("{:?}",user_cmd);
        match user_cmd[0] {
            "add" =>{
                let error = "Error";
                let name = user_cmd.get(1).unwrap_or(&error);
                let department = user_cmd.get(3).unwrap_or(&error);
                database.insert(name.to_string(),department.to_string());
                names_table.push(name.to_string());
                println!("add {} to {} successful",name,department)
            },
            "ls" =>{
                println!("Name\tDepartment");
                names_table.sort();
                let department_in_cmd = user_cmd.get(1).unwrap_or(&"Error");
                for (name,department) in &database{
                    if department[..] == **department_in_cmd {
                        println!("{}\t{}",name,department);

                    }
                }
            },
            "ls_all" =>{
                println!("Name\tDepartment");
                names_table.sort();
                for name in &names_table{
                    let error = "Error".to_string();
                    let department = database.get(name).unwrap_or(&error);
                    println!("{}\t{}",name,department)
                }
            },
            "exit" => break,
            _=> println!("Unknown Command"),
        }

    }




}
