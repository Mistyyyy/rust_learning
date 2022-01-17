fn main() {
    let mut str = String::new();
    let mut vec: Vec<String> = vec![];

    {
        str = String::from("1");

        vec.push(str);

        str = String::from("2");

        vec.push(str);

        str = String::from("");

        println!("{}, {:#?}", str, vec);
    }
}
