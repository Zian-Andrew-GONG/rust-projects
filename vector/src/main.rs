#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    println!("Hello, world!");
    {
        let v: Vec<i32> = Vec::new();
        let v = vec![1, 2, 3];
    }
    {
        let mut v = Vec::new();
        v.push(5);
    }
    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        // get 方法
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }
    {
        let v = vec![1, 2, 3, 4, 5];
        // let does_not_exist = &v[100]; // panic!
        let does_not_exist = v.get(100);
    }
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        for i in v {
            println!("{}", i);
        } // v is moved
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}
