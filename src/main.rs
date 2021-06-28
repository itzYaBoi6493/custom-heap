mod pqueue;

use std::cmp::Ordering;
use pqueue::Heap;


fn main() {
    let mut h: Heap<i32> = Heap::new(|i, j| {
        if *i < *j {
            Ordering::Greater
        } else if *i == *j {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    for i in 1..=10 {
        h.insert(i);
    }

    while !h.is_empty() {
        print!("{} ", h.extract_min().unwrap());
    }
    println!();

    println!("heap: {:?}", h.get_size());
}
