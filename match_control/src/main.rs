#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    println!("Hello, world!");
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        value_in_cents(Coin::Penny);
    }
    {
        #[derive(Debug)] // 这样可以立刻看到州的名称
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
        value_in_cents(Coin::Quarter(UsState::Alabama));
    }
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {
            println!("other");
        }
    }
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {
            println!("_");
        }
    }
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (), // 无事发生
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
    }
    // if let 代替 match
    {
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
        }
        let some_u8_value = Some(3u8);
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }
}
