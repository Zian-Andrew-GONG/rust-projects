#![allow(dead_code)]
#![allow(unused_variables)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn panic_when_negative(x: i32) {
    if x < 0 {
        panic!("should not be negative, get {}", x);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    #[ignore]
    fn seven_adds_two() {
        assert_eq!(9, add(7, 2));
        assert_eq!(6, add(7, 2), "add result should be {}", 9);
        assert_ne!(5, add(7, 2));
    }
    #[test]
    #[should_panic(expected = "should not be negative")]
    fn should_panic() {
        panic_when_negative(-1);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
