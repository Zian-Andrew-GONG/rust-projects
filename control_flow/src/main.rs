fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" };
    println!("The value of number is: {}", number);

    // loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // break loop with value
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    println!("fab = {}, fab1 = {}", fab(10), fab1(10));
}

fn fab(n: i32) -> i32 {
    if n == 1 || n == 2 {
        1
    } else {
        fab(n - 1) + fab(n - 2)
    }
}

fn fab1(n: i32) -> i32 {
    let mut pre_1 = 1;
    let mut pre_2 = 1;
    let mut curr = 1;
    if n <= 2 {
        return 1;
    }
    let mut i = 3;
    while i <= n {
        curr = pre_1 + pre_2;
        pre_1 = pre_2;
        pre_2 = curr;
        i = i + 1;
    }
    curr
}
