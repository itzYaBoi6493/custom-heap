use std::cmp::Ordering;

pub struct Heap<T> {
    arr: Vec<T>,
    comp: Box<dyn Fn(&T, &T) -> Ordering>,
    n: usize,
}

impl<T> Heap<T>
    where T: std::fmt::Display
{
    pub fn new<F>(comp: F) -> Self
        where F: Fn(&T, &T) -> Ordering + 'static
    {
        Self {
            arr: Vec::<T>::new(),
            comp: Box::new(comp),
            n: 0usize,
        }
    }

    pub fn get_size(&self) -> usize {
        self.n
    }

    pub fn insert(&mut self, data: T) {
        // append the data into the array
        self.arr.push(data);
        // data is moved into the heap/array
        self.n += 1;
        // increment size
        self.bubble_up(self.n - 1);

        // Ordering::Less will bubble up
        // so, extract_min would be the name of the function.
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn extract_min(&mut self) -> Result<T, ()> {
        match self.arr.get(0) {
            None => {
                Err(())
            },
            Some(_) => {
                // swap
                self.arr.swap(0, self.n - 1);
                let ret: T = self.arr.pop().unwrap();
                self.n -= 1;
                self.bubble_down(0);
                Ok(ret)
            }
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        if self.get_size() <= index {
            return;
        }
        // less should be at top
        // so, comparing children should result
        // in the return of the minimum of them
        let mut lc = self.left_child(index);
        let mut rc = self.right_child(index);
        let mut min_child = if let Ordering::Less = (self.comp)(&self.arr[lc], &self.arr[rc]) {
            // comparing lc with rc
            // if arr[lc] < arr[rc]
            lc
        } else {
            rc
        };

        while let Ordering::Greater = (self.comp)(&self.arr[index], &self.arr[min_child]) {
            // if parent '>' min_child,
            // swap
            self.arr.swap(index, min_child);

            index = min_child;
            lc = self.left_child(index);
            rc = self.right_child(index);
            min_child = if let Ordering::Less = (self.comp)(&self.arr[lc], &self.arr[rc]) {
                // comparing lc with rc
                // if arr[lc] < arr[rc]
                lc
            } else {
                rc
            };
        }
    }

    fn bubble_up(&mut self, mut index: usize) {
        if self.get_size() <= index {
            return;
        }
        // find parent
        let mut p = self.parent(index);
        // compare parent and child
        while let Ordering::Greater = (self.comp)(&self.arr[p], &self.arr[index]) {
            // p '>' index
            // swap parent and child
            self.arr.swap(p, index);

            index = p;
            p = self.parent(index);
        }
    }

    fn parent(&self, i: usize) -> usize {
        if i == 0 || i % 2 != 0 {
            i / 2
        } else {
            i / 2 - 1
        }
    }

    fn right_child(&self, i: usize) -> usize {
        let rc = 2 * i + 2;
        if rc >= self.n {
            i
        } else {
            rc
        }
    }

    fn left_child(&self, i: usize) -> usize {
        let lc = 2 * i+ 1;
        if lc >= self.n {
            i
        } else {
            lc
        }
    }
}
