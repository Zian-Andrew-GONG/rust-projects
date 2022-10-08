#![allow(unused_variables)]
#![allow(unused_assignments)]
fn main() {
    {
        let mut s = String::new();
        let data = "initial contents";
        s = data.to_string();
        let s = "initial contents".to_string();
        let s = String::from("initial contents");
    }
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2); // s2 is not moved
        println!("s2 is {}", s2);
        let mut s = String::from("lo");
        s.push('l');
    }
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        // println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);  // s1 is moved

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
    }
    {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }
        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}
