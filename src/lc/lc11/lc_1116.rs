// https://leetcode.com/problems/print-zero-even-odd/
// 1116. Print Zero Even Odd
pub struct ZeroEvenOdd {
    n: i32,
    cv: std::sync::Condvar,
    mtx: std::sync::Mutex<i32>,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        ZeroEvenOdd {
            n,
            cv: std::sync::Condvar::new(),
            mtx: std::sync::Mutex::new(0),
        }
    }

    // printNumber(x) prints the integer x
    fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n {
            let mut mtx = self.mtx.lock().unwrap();
            while *mtx % 2 != 0 {
                mtx = self.cv.wait(mtx).unwrap();
            }
            print_number(0);
            *mtx += 1;
            self.cv.notify_all();
        }
    }

    fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..self.n / 2 {
            let mut mtx = self.mtx.lock().unwrap();
            while *mtx % 4 != 3 {
                mtx = self.cv.wait(mtx).unwrap();
            }
            print_number(*mtx / 2 + 1);
            *mtx += 1;
            self.cv.notify_all();
        }
    }

    fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for _ in 0..(self.n + 1) / 2 {
            let mut mtx = self.mtx.lock().unwrap();
            while *mtx % 4 != 1 {
                mtx = self.cv.wait(mtx).unwrap();
            }
            print_number(*mtx / 2 + 1);
            *mtx += 1;
            self.cv.notify_all();
        }
    }
}
unsafe impl Send for ZeroEvenOdd {}
unsafe impl Sync for ZeroEvenOdd {}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn foobar() {
        for it in [1, 2] {
            let zero_even_odd = Arc::new(ZeroEvenOdd::new(it));
            let shared_vector = Arc::new(Mutex::new(vec![]));
            let v1 = Arc::clone(&shared_vector);
            let zero_even_odd0 = Arc::clone(&zero_even_odd);
            let handle0 = std::thread::spawn(move || zero_even_odd0.zero(|i| v1.lock().unwrap().push(i)));
            let v1 = Arc::clone(&shared_vector);
            let zero_even_odd1 = Arc::clone(&zero_even_odd);
            let handle1 = std::thread::spawn(move || zero_even_odd1.even(|i| v1.lock().unwrap().push(i)));
            let v2 = Arc::clone(&shared_vector);
            let zero_even_odd2 = Arc::clone(&zero_even_odd);
            let handle2 = std::thread::spawn(move || zero_even_odd2.odd(|i| v2.lock().unwrap().push(i)));
            handle0.join().unwrap();
            handle1.join().unwrap();
            handle2.join().unwrap();
            assert_eq!(
                *shared_vector.lock().unwrap(),
                (1..=it).map(|i| [0, i]).flatten().collect::<Vec<_>>()
            );
        }
    }
}
