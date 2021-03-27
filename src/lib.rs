#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Fuck");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 8,
        };
        let smaller = Rectangle {
            width: 6,
            height: 7,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 5,
            height: 5,
        };
        let smaller = Rectangle {
            width: 4,
            height: 4,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn it_greets_carol() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        )
    }

    #[test]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    #[should_panic]
    fn guess_to_big() {
        Guess::new(102);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Oh neini!"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(number: i32) -> i32 {
    number + 2
}

pub fn greeting(name: &str)  -> String {
    format!("Hello {}!", name)
    // String::from("Hello")
}

struct Guess {
    value: i32,
}


/*
failures:

---- tests::guess_to_big stdout ----
note: test did not panic as expected
 */
impl Guess {
    pub fn new(number: i32) -> Guess {
        // if number < 1 || number > 100 {
        // Switching the if's so that they use the wrong panic message.
        // The panic message has to mathc to the expected substring in the should_panic attribute.
        if number > 100 /* || number > 100 */{
            panic!("Guess value must be greater than or equal to 1, got {}.", number);
        }
        if number < 1  {
            panic!("Guess value must be less than or equal to 100, got {}.", number);
        }
        Guess {
            value: number,
        }
    }
}
