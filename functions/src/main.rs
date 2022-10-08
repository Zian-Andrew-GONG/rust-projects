fn another_function(x: i32) {
    println!("Another function. x = {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn return_five() -> i32 {
    return 5;
}

fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(3, 's');

    // rust 赋值不能写成 x = y = 6 的形式，因为赋值运算符没有返回值
    // let x = (let y = 6);

    let y = {
        let x = 3;  // 语句
        x + 1  // 表达式
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("x = {}", x);
    let x = return_five();
    println!("x = {}", x);
}
