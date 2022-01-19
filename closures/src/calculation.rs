use std::thread;
use std::cmp::Eq;
use std::hash::Hash;
use std::time::Duration;
use std::collections::HashMap;

struct Cache<T, U>
    where
        T: Fn(U) -> U
{
    calculation: T,
    cacheMap: HashMap<U, U>
}

impl<T, U> Cache<T, U>
    where
        U: Copy + Eq + Hash,
        T: Fn(U) -> U
{
    pub fn new(calculation: T) -> Cache<T, U> {
        Cache {
            calculation,
            cacheMap: HashMap::new()
        }
    }

    pub fn value(&mut self, arg: U) -> U {
        match self.cacheMap.get(&arg) {
            Some(v) => *v,
            None => {
                let value = (self.calculation)(arg);
                self.cacheMap.insert(arg, value);
                value
            }
        }
    }
}

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");

    thread::sleep(Duration::from_secs(2));

    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_closure = Cache::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });


    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cache::new(|a| a);

    c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}