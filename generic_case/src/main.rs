mod largest;
mod apply;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest::largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest::largest_char(&char_list);
    println!("The largest char is {}", result);
}
