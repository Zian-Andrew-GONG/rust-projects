use advanced_traits::{Human, Pilot, Wizard};
use std::ops::Add;
fn main() {
    {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        impl Add for Point {
            type Output = Point;
            fn add(self, rhs: Self) -> Self::Output {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }
    {
        #[derive(Debug, PartialEq)]
        struct Millimeters(u32);
        struct Meters(u32);
        impl Add<Meters> for Millimeters {
            type Output = Millimeters;
            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }
        assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000));
    }
    {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }
    {
        trait Animal {
            fn baby_name() -> String;
        }
        struct Dog;
        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }
        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }
        println!("A baby dog is called a {}", Dog::baby_name());
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }
    {
        use std::fmt;
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string(); // 使用了 Display trait
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }
        struct Point {
            x: i32,
            y: i32,
        }
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        impl OutlinePrint for Point {}
        let p = Point { x: 100, y: 13 };
        p.outline_print();
    }
    {
        use std::fmt;
        struct Wrapper(Vec<String>);
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}
