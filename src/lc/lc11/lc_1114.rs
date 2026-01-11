// https://leetcode.com/problems/print-in-order/
// 1114. Print in Order
pub struct Foo {
    cv: std::sync::Condvar,
    mtx: std::sync::Mutex<usize>,
}
unsafe impl Send for Foo {}
unsafe impl Sync for Foo {}
impl Foo {
    pub fn new() -> Self {
        Foo {
            cv: std::sync::Condvar::new(),
            mtx: std::sync::Mutex::new(0),
        }
    }

    pub fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        // Do not change this line
        print_first();
        *self.mtx.lock().unwrap() += 1;
        self.cv.notify_all();
    }

    pub fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        let mut mtx = self.mtx.lock().unwrap();
        while *mtx != 1 {
            mtx = self.cv.wait(mtx).unwrap();
        }
        // Do not change this line
        print_second();
        *mtx += 1;
        self.cv.notify_all();
    }

    pub fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        let mut mtx = self.mtx.lock().unwrap();
        while *mtx != 2 {
            mtx = self.cv.wait(mtx).unwrap();
        }
        // Do not change this line
        print_third();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::sync::{Arc, Mutex};
    #[test]
    fn foo() {
        let data = vec![1, 2, 3];
        for perm in data.into_iter().permutations(3) {
            let foo = Arc::new(Foo::new());
            let mut handles = vec![];
            let shared_vector = Arc::new(Mutex::new(vec![]));
            for i in perm.into_iter() {
                let v = Arc::clone(&shared_vector);
                let foo = Arc::clone(&foo);
                handles.push(std::thread::spawn(move || match i {
                    1 => foo.first(|| {
                        v.lock().unwrap().push(1);
                    }),
                    2 => foo.second(|| {
                        v.lock().unwrap().push(2);
                    }),
                    3 => foo.third(|| {
                        v.lock().unwrap().push(3);
                    }),
                    _ => unreachable!(),
                }));
            }
            handles.into_iter().map(|h| h.join()).collect_vec();
            assert_eq!(*shared_vector.lock().unwrap(), vec![1, 2, 3]);
        }
    }
}
