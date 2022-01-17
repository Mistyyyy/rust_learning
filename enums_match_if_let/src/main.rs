fn main() {
    println!("Hello, world!");
    let cents = values_in_cents(Coin::Quarter(UsState::Alabama));
    println!("The cents is {}", cents);
    let values = Some(5);
    let some_in_values8 = Some(0u8);
    plus_one(values);
    let none = None;
    plus_one(none);

    dice_roll();
    dice_roll_other();

    if let Some(5) = some_in_values8 {
        println!("Five!!!");
    } else {
        println!("Not Five!!!");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn values_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        } 
    }
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        Some(t) => Some(t + 1),
        None => None
    }
}

fn dice_roll() {
    let dices = 4;
    match dices {
        3 => add_fancy_hat(),
        5 => remove_fancy_hat(),
        other => move_player(other)
    }
}

fn dice_roll_other() {
    let dices = 4;
    match dices {
        3 => add_fancy_hat(),
        5 => remove_fancy_hat(),
        _ => ()
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}