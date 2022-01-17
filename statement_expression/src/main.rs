fn main() {
   let number = 5;
   if number % 2 == 1 {
       println!("The statement is True");
   } else {
       println!("The statement is False");
   }

   let is_odd = if number % 2 == 1 {
       true
   } else {
       false
   };

   println!("The result is {}", is_odd);
//    loop_fn();

   loop_break();

   loop_return_value();
}

fn loop_fn() {
    loop {
        println!("Loop");
    }
}

fn loop_break() {
    let mut count = 0;
    'count_loop: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'count_loop;
            }

            remaining = remaining - 1;
        }

        count = count + 1;
    }
}

fn loop_return_value() {
    let mut count = 0;
    let result = loop {
        count = count + 1;
        if count == 2 {
            break count * 2 + 1;
        }
    };

    println!("The result is {}", result);

    for_statement();
    for_numner();
}

fn for_statement() {
    let filter = [1, 2, 3];
    for element in filter.iter() {
        println!("the value is: {}", element)
    }
}

fn for_numner() {
    for number in (1..4).rev() {
        println!("{}!", number)
    }
}