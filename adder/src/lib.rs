#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "faile")]
    fn another() {
        panic!("failed");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = ReactAngle { width: 8, height: 7 };
        let smaller = ReactAngle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = ReactAngle { width: 8, height: 7 };
        let smaller = ReactAngle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(3, 5), 8)
    }

    #[test]
    fn test_greeting() {
        let string = greeting("carol");
        assert!(
            string.contains("carol"),
            "Greeting did not contain name, value was `{}`",
            string
        );
    }

    #[test]
    fn it_should_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

struct ReactAngle {
    width: u32,
    height: u32,
}

impl ReactAngle {
    fn can_hold(&self, other: &ReactAngle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: u32, b: u32) -> u32 {
    a + b + 3
}

fn greeting(str: &str) -> String {
    format!("Hello {}", str)
}
