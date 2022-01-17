fn main() {
    version1();
    version2();
    version3();
}

fn version1() {
    let width1 = 30;
    let height1 = 40;
    println!(
        "The area of the rectangle is {} square pixels.",
        areas1(width1, height1)
    );
}

fn version2() {
    let rect = (30, 40);
    println!(
        "The area of the rectangle is {} square pixels.",
        areas2(rect)
    );
}

fn version3() {
    let scale = 2;
    let rect = ReactAngle {
        height: dbg!(30 * scale),
        width: 40
    };

    let rect1 = ReactAngle::square(25);

    println!(
        "The area of the rectangle is {} square pixels.{},{:#?}",
        areas3(&rect),
        rect.height,
        rect
    );

    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("The react {:#?} can hold rect {:#?} is {}", rect, rect1, rect.can_hold(&rect1));

    // dbg!(&rect);
    
}

fn areas1(width: u32, height: u32) -> u32 {
    width * height
}

fn areas2(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

#[derive(Debug)]
struct ReactAngle {
    width: u32,
    height: u32,
}

struct R(u32);

impl ReactAngle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, react: &ReactAngle) -> bool {
        self.height >= react.height && self.width >= react.width
    }

    fn square(size: u32) -> ReactAngle {
        ReactAngle {
            width: size,
            height: size
        }
    }
}

fn areas3(react_angle: &ReactAngle)-> u32 {
    let m = R(1);
    println!("{}", m.0);
    react_angle.height * react_angle.width
}