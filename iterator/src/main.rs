fn main() {
    {
        let v1 = vec![1, 2, 3];

        for val in &v1 {
            println!("Got: {}", val);
        }
        println!("------");

        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
        println!("------");

        let mut v1_iter = v1.iter();
        println!("Got: {}", v1_iter.next().unwrap());
        println!("Got: {}", v1_iter.next().unwrap());
        println!("Got: {}", v1_iter.next().unwrap());
        println!("Got: {}", v1_iter.next().unwrap_or_else(|| &-1));
        println!("------");

        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        println!("total = {}", total);
        println!("------");

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        for val in &v2 {
            println!("Got: {}", val);
        }
        println!("------");
    }
    {
        let v1 = vec![1, 2, 3];
        println!("v1 = {:?}", v1);
        let mut v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        println!("v1 = {:?}", v1);
        println!("v2 = {:?}", v2);
        let v3: Vec<_> = v2
            .iter_mut()
            .map(|x| {
                *x += 1;
                x
            })
            .collect();
        println!("v1 = {:?}", v1);
        println!("v3 = {:?}", v3);
        println!("v2 = {:?}", v2);

        let v4: Vec<_> = v1.into_iter().map(|x| x + 1).collect();
        // println!("v1 = {:?}", v1);  // v1 moved
        println!("v4 = {:?}", v4);
    }
}
