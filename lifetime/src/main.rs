/*
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
 */
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::fmt::Display;
fn main() {
    {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("abcd");
        {
            let string2 = "xyz";
            let result = longest(string1.as_str(), string2);
            println!("The longest string is {}", result);
        }
    }
    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        let i;
        {
            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            println!("{}", first_sentence);
            i = ImportantExcerpt {
                part: first_sentence,
            };
        }
        // let j = i; // Error
    }
    {
        let s: &'static str = "I have a static lifetime.";
    }
    {
        // use generics, traits and liftime
        pub trait Print {
            fn print(&self);
        }
        pub struct PrS<T> {
            value: T,
        }
        impl<T: Display> Print for PrS<T> {
            fn print(&self) {
                println!("self: {}", self.value);
            }
        }
        fn print_tw_return_first<'a,'b, T: Display>(a: &'a T, b: &'b T)-> &'a T {
            println!("a: {}, b:{}", a, b);
            a
        }
    }
    // println!("{}", s);
}
