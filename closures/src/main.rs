mod calculation;

fn main() {
    let x = 4;

    let equal_to_x = |m: i32| m == x;

    let move_equal_to_x = move |m: i32| m == x;

    println!("Hello, world!", x);

    calculation::generate_workout(40, 7)
}
