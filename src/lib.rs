pub mod pqueue;

#[cfg(test)]
mod tests {
    use super::pqueue::Heap;
    #[test]
    fn test_new() {
        let h: Heap<i32> = Heap::new(|i: &i32, j: &i32| {
            i.cmp(j)
        });

        assert_eq!(h.get_size(), 0);
        assert!(h.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut h: Heap<i32> = Heap::new(|i: &i32, j: &i32| {
            j.cmp(&i)
        });

        let mut v = Vec::<i32>::new();
        for i in 1..=10 {
            h.insert(i);
            println!("{:?}", h.arr);
        }

        while !h.is_empty() {
            v.push(h.extract_min().unwrap());
            println!("{:?}", h.arr);
        }

        let mut next = &v[0];
        for i in &v {
            if *next < *i {
                assert!(false);
            }
            next = i;
        }
        assert!(true);
    }
}
