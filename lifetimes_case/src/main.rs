mod longest;
mod structure;

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest::longest(string1.as_str(), string2.as_str());
        structure::longest_with_an_announcement(string1.as_str(), string2.as_str(), &string1);
        println!("The longest string is {}", result);
    }

    structure::generate_excerpt();
    // println!("The longest string is {}", result);
}
