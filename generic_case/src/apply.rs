pub struct Point<T, U> {
    x: T,
    y: U
}

impl<T: Copy, U: Copy> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(&self, other: &Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn use_point() {
    let integer = Point { x: 3, y: 2 };
    let float = Point { x: 3.1, y: 3.2 };
    let string = Point { x: String::from("111"), y: String::from("222") };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(&p2);


    println!("p.x = {}", integer.x());

    println!("p.x = {}", string.x());

    println!("p.distance = {}", float.distance_from_origin());

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}