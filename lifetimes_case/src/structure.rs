
use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn generate_excerpt<'a>() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let part = novel.split('.').next().expect("err");
    let c = ImportantExcerpt {
        part
    };

    println!("The structure is {:?}", &c);
}

pub fn longest_with_an_announcement<'a, T>(str1: &'a str, str2: &'a str, ann: T) -> &'a str 
    where
        T: Display
{
    println!("Announcement! {}", ann);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}