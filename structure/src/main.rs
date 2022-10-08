#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    println!("Hello, world!");
    {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        // user1.email = String::from("anotheremail@example.com");
    }
    {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        user1.email = String::from("anotheremail@example.com");
    }
    {
        build_user(
            String::from("someone@example.com"),
            String::from("someusername123"),
        );
    }
    {
        let user_1 = build_user_1(
            String::from("someone@example.com"),
            String::from("someusername123"),
        );
        let user_2 = User {
            active: user_1.active,     // move
            username: user_1.username, // move
            email: String::from("anotheremail@example.com"),
            sign_in_count: user_1.sign_in_count,
        };
        println!("email: {}", user_1.email);
    }
    {
        let user_1 = build_user_1(
            String::from("someone@example.com"),
            String::from("someusername123"),
        );
        let user_2 = User {
            email: String::from("anotheremail@example.com"),
            ..user_1
        };
        // println!("email: {}", user_1.username);
        println!("email: {}", user_1.email);
    }
    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
    {
        struct AlwaysEqual;
        let subject_1 = AlwaysEqual;
        let subject_2 = AlwaysEqual;
    }
    {
        println!("area = {}", area(1, 2));
        println!("area_1 = {}", area_1((1, 2)));
        println!(
            "area_2 = {}",
            area_2(Rectangle {
                width: 1,
                height: 2
            })
        );
    }
    {
        let rectangle = Rectangle {
            width: 1,
            height: 2,
        };
        println!("rec: {:#?}", rectangle);
    }
    {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale), // dbg! 接收表达式的所有权并返回表达式的值的所有权
            height: 50,
        };
        dbg!(&rect1);
    }
    {
        let rectangle = Rectangle {
            width: 1,
            height: 2,
        };
        println!("area = {}", rectangle.area());
        let mut rec = rectangle;
        rec.change_width(3);
        println!("area = {}", rec.area());
        rec.clear();
        // println!("area = {}", rec.area()); // Error
    }
    {
        let square = Rectangle::square(5);
        println!("area = {}", square.area());
    }
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
fn build_user_1(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
// doesn't work ...
// fn build_user_2(email_: String, username_: String) -> User {
//     User {
//         email_,
//         username_,
//         active: true,
//         sign_in_count: 1,
//     }
// }

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_1(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area_2(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
    fn clear(self) {}
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
