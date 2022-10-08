fn main() {
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length_(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }
    {
        let s = String::from("hello");
        change(&s);
        let mut s_ = String::from("hello");
        change_(&mut s_);
    }
    // 在同一时间，只能有一个对某一特定数据的可变引用。
    {
        // let mut s = String::from("hello");
        // let r1 = &mut s;
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
    }
    // 也不能在拥有不可变引用的同时拥有可变引用
    {
        // let mut s = String::from("hello");
        // let r1 = &s; // 没问题
        // let r2 = &s; // 没问题
        // let r3 = &mut s; // 大问题
        // println!("{}, {}, and {}", r1, r2, r3);
    }
    {
        let _reference_to_nothing = dangle();
    }
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用

//     let s = String::from("hello"); // s 是一个新字符串

//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
//   // 危险！
fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn calculate_length_(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

fn calculate_length(s: &String) -> usize {
    // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

fn change(s: &String) {
    println!("cannot change, string is {}", s);
}
fn change_(s: &mut String) {
    s.push_str(", world");
    println!("changed, string is {}", s);
}
