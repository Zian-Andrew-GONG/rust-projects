use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let mut x = 6;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    let _guess: u32 = "42".parse().expect("Not a number!");

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

    // bool
    let _t = true;
    let _f: bool = false;

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {};\nz: {};\nheart_eyed_cat: {}", c, z, heart_eyed_cat);

    // tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("x = {}, y = {}, z = {}", tup.0, tup.1, tup.2);

    // array
    let _a = [1, 2, 3, 4, 5];
    let _b = [3; 6];

    // an example, 当index 大于 4 时会立即退出，而不是像 C++ 一样访问无效内存
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
