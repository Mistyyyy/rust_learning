pub fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

// pub fn first_word<'a>(str1: &'a str, str2: &str) -> &'a str {
//     str1
// }

// pub fn longest_1<'a>(str1: &str, str2: &str) -> &'a str {
//     let result = String::from("this is string");
//     result.as_str()
// }