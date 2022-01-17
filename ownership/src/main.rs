fn main() {
    let s = String::from("this is string");
    let s2 = s.clone();
    println!("{}{}", s, s2);

    let st = String::from("This is another string");
    take_ownership(st);

    // println!("{}", st); // error

    let a = 1;
    make_copy(a);
    println!("{}", a);

    let s3 = String::from("This is another string");
    let (s4, len) = calc_length(s3);
    println!("{}{}", len, s4);

    let s5 = String::from("This is another string1");
    let lens = calc_length_new(&s5[..]);
    println!("{}", lens);

    let mut s6 = String::from("this is another string2");
    let p1 = &mut s6;
    change(p1);

    let s7 = String::from("This is another string1");
    let w = first_word(&s7);
    println!("{}", w);

    let s8 = String::from("This is another string3");
    let w = first_word_new(&s8);
    println!("{}", w);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_int: i32) {
    println!("{}", some_int);
}

fn calc_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

fn calc_length_new(some_string: &str) -> usize {
    some_string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", this is append");
    println!("{}", some_string);
}

fn first_word(word: &String) -> usize {
    let bytes = word.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }
    word.len()
}

fn first_word_new(word: &str) -> &str {
    let bytes = word.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &word[0..index];
        }
    }
    &word[..]
}