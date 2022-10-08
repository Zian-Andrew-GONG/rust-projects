#![allow(unused_variables)]
#![allow(dead_code)]

use std::mem::swap;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largest_<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }
        impl<T, U> Point<T, U> {
            fn x(&self) -> &T {
                &self.x
            }
            fn y(&self) -> &U {
                &self.y
            }
        }
        impl Point<i32, i32> {
            fn exchange(&mut self) {
                swap(&mut self.x, &mut self.y);
            }
        }
        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: "h", y: 4.0 };
        let mut p2 = Point { x: 5, y: 0 };
        let p3 = Point { x: 5, y: "s" };
        println!("x = {}, y = {}", p3.x(), p3.y());
        p2.exchange();
        println!("x = {}, y = {}", p2.x(), p2.y());
        let p3 = p1.mixup(p2);


    }
}
