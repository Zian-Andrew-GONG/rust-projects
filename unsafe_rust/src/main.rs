use std::slice;
fn main() {
    {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        println!("r1 is:{:?}", r1);
        println!("r2 is:{:?}", r2);
        unsafe {
            println!("*r1 is:{}", *r1);
            println!("*r2 is:{}", *r2);
        }
        // let address = 0x012345usize;
        // let r = address as *const i32;
        // unsafe {
        //     println!("*r is: {}", *r); // segmentation fault
        // }
    }
    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();
            assert!(mid <= len);
            // (&mut slice[..mid], &mut slice[mid..])
            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    {
        extern "C" {
            fn abs(input: i32) -> i32;
        }
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }
    {
        static mut COUNTER: u32 = 0;
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }
        add_to_count(3);
        unsafe {
            println!("COUNTER = {}", COUNTER);
        }
    }
}
