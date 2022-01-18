pub fn largest<T: PartialOrd + Copy>(target: &[T]) -> T {
    let mut max = target[0];

    for &num in target.iter() {
        if num > num {
            max = num;
        }
    }

    max
}

pub fn another_largest<T: PartialOrd>(target: &[T]) -> &T {
    let mut max = &target[0];

    for num in target.iter() {
        if num > num {
            max = num;
        }
    }

    max
}

pub fn largest_i32(target: &[i32]) -> i32 {
    let mut max = target[0];

    for &item in target.iter() {
        if item > max {
            max = item
        }
    }

    max
}

pub fn largest_char(target: &[char]) -> char {
    let mut max = target[0];

    for &item in target.iter() {
        if item > max {
            max = item
        }
    }

    max
}