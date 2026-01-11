// https://leetcode.com/problems/print-foobar-alternately/
// 1115. Print in Order
pub struct FooBar {
    n: usize,
    cv: std::sync::Condvar,
    mtx: std::sync::Mutex<usize>,
}
impl FooBar {
    pub fn new(n: usize) -> Self {
        FooBar {
            n,
            cv: std::sync::Condvar::new(),
            mtx: std::sync::Mutex::new(0),
        }
    }
    pub fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut mtx = self.mtx.lock().unwrap();
            while *mtx != 0 {
                mtx = self.cv.wait(mtx).unwrap();
            }
            // printFoo() outputs "foo". Do not change or remove this line.
            print_foo();
            *mtx ^= 1;
            self.cv.notify_all();
        }
    }

    pub fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut mtx = self.mtx.lock().unwrap();
            while *mtx != 1 {
                mtx = self.cv.wait(mtx).unwrap();
            }
            // printBar() outputs "bar". Do not change or remove this line.
            print_bar();
            *mtx ^= 1;
            self.cv.notify_all();
        }
    }
}
unsafe impl Send for FooBar {}
unsafe impl Sync for FooBar {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn foobar() {
        for it in [1, 2] {
            let foobar = Arc::new(FooBar::new(it));
            let shared_vector = Arc::new(Mutex::new(vec![]));
            let v1 = Arc::clone(&shared_vector);
            let foobar1 = Arc::clone(&foobar);
            let handle1 = std::thread::spawn(move || foobar1.foo(|| v1.lock().unwrap().push(0)));
            let v2 = Arc::clone(&shared_vector);
            let foobar2 = Arc::clone(&foobar);
            let handle2 = std::thread::spawn(move || foobar2.bar(|| v2.lock().unwrap().push(1)));
            handle1.join().unwrap();
            handle2.join().unwrap();
            assert_eq!(*shared_vector.lock().unwrap(), vec![0, 1].repeat(it));
        }
    }
}
