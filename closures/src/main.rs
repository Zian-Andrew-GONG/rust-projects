use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T, Input, Output>
where
    T: Fn(Input) -> Output,
{
    calculation: T,
    map: HashMap<Input, Output>,
}

impl<T> Cacher<T, u32, u32>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T, u32, u32> {
        Cacher {
            calculation: calculation,
            map: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.map.get(&arg) {
            Some(v) => *v,
            None => {
                let value = (self.calculation)(arg);
                self.map.insert(arg, value);
                value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculate slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    {
        println!("...");
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));

        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];    
        assert!(equal_to_x(y));
    
    }
}
