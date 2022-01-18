mod self_vec;
mod self_string;
mod self_hashmap;
mod quiz;

use std::io;
use std::collections::HashMap;

fn main() {
    let mut my_vec = self_vec::create_new_vec();
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);

    println!("this is vec {:?}", my_vec);
    let mut another_vec = self_vec::create_initial_vec();

    println!("this is another vec {:?}", another_vec);

    let third: &i32 = &my_vec[2];

    // my_vec.push(4);

    println!("this is third element: {}", third);

    another_vec.push(5);

    match another_vec.get(3) {
        Some(third) => println!("The third element id {}", third),
        None =>  println!("There is no third element."),
    }


    for i in &my_vec {
        println!("{}", i);
    }

    for i in &mut my_vec {
        *i += 1;
    }

    println!("this is vec {:?}", my_vec);

    let mut enum_vec = self_vec::create_enum_vec();

    println!("this is enum vec {:?}", enum_vec);

    enum_vec.push(self_vec::SpreadSellCell::Text(String::from("another text")));

    println!("this is enum vec {:?}", enum_vec);

    let str = "this is string".to_string();

    // let m = &str[0..2];

    // let str1 = "111";

    println!("{}", str);

    let mut str1 = String::from("Hello ");
    str1.push_str("world");
    println!("{}", str1);

    str1.push_str(&str);
    println!("{}, {}", str1, str);

    let str2 = str1 + &str;
    println!("{}", str2);

    // let chars = &str[0]; 字符串不支持索引取字符

    for b in str.chars() {
        println!("{}", b);
    }

    for b in str.bytes() {
        println!("{}", b);
    }

    let mut maps: HashMap<String, i32> = HashMap::new();

    maps.insert(String::from("Blue"), 1);
    maps.insert(String::from("Red"), 2);

    println!("The map is {:?}", maps);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![10, 20];
    let team_scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    println!("The map is {:?}", team_scores);

    let target_team = String::from("Blue");
    let score = team_scores.get(&target_team);
    println!("The team {} score is {:?}", target_team, score);

    for (k, v) in &team_scores {
        println!("{}, {}", k, v);
    }

    let mut ao_maps: HashMap<String, i32> = HashMap::new();

    ao_maps.entry(String::from("Blue")).or_insert(20);

    let text = "hello world wonderful world";
    let mut counter_map: HashMap<&str, i32> = HashMap::new();

    for ch in text.split_whitespace() {
        let count = counter_map.entry(ch).or_insert(0);
        *count += 1;
    }

    println!("{:?}", counter_map);

    let mut v1 = vec![1, 2, 3];

    // let a = v1[0];

    v1.push(3);

    let val = quiz::op_number_vec(&v1);

    println!("{:?}, {:?}", val, v1);

    add_employee_to_department();
}

fn add_employee_to_department() {
    let mut empolyee_department_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut employee;
    let mut department;
    let mut contine_input;

    loop {
        employee = String::new();
        department = String::new();
        contine_input = String::new();

        println!("Please input the employee name");

        loop {
            io::stdin().read_line(&mut employee).expect("please input the empolyee");

            if employee.trim() == "" {
                println!("please input the valid name");
                continue;
            } else {
                break;
            }
        };

        println!("Please input the department name");

        loop {
            io::stdin().read_line(&mut department).expect("please input the department");

            if department.trim() == "" {
                println!("please input the valid name");
                continue;
            } else {
                break;
            }
        };

        let empolyees = empolyee_department_map.entry(department.trim().to_owned()).or_insert(vec![]);
        empolyees.push(employee.trim().to_owned());
        empolyees.sort();
        empolyees.dedup();

        println!("employee is {}, department is {}", employee, department);


        println!("input continue ?[y/n]");

        io::stdin().read_line(&mut contine_input).expect("please input the signal");

        if &contine_input.trim().to_lowercase() == "y" || &contine_input.trim().to_lowercase() == "yes" {
            continue;
        } else {
            break;
        }
    }
    println!("the department table is {:#?}", empolyee_department_map)
}
