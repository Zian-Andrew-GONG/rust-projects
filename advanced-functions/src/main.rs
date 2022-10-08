fn main() {
    {
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
        let answer = do_twice(add_one, 5);
        println!("The answer is {}", answer);
    }
    {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_numbers: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        println!("list of numbers: {:?}", list_of_numbers);

        let list_of_numbers = vec![1, 2, 3];
        let list_of_numbers: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
        println!("list of numbers: {:?}", list_of_numbers);
    }
    {
        fn return_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
        println!("result = {}", return_closure()(1));
    }
}
