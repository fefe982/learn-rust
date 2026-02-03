// https://leetcode.com/problems/the-dining-philosophers/
// 1226. The Dining Philosophers
pub struct DiningPhilosophers {
    cv: std::sync::Condvar,
    mtx: std::sync::Mutex<i32>,
    forks: Vec<std::sync::Mutex<i32>>,
}

impl DiningPhilosophers {
    pub fn new() -> Self {
        DiningPhilosophers {
            cv: std::sync::Condvar::new(),
            mtx: std::sync::Mutex::new(0),
            forks: vec![
                std::sync::Mutex::new(1),
                std::sync::Mutex::new(1),
                std::sync::Mutex::new(1),
                std::sync::Mutex::new(1),
                std::sync::Mutex::new(1),
            ],
        }
    }

    // Callbacks are like LeetCode: each used exactly once
    pub fn wants_to_eat<F1, F2, F3, F4, F5>(
        &self,
        philosopher: i32,
        pick_left_fork: F1,
        pick_right_fork: F2,
        eat: F3,
        put_left_fork: F4,
        put_right_fork: F5,
    ) where
        F1: FnOnce(),
        F2: FnOnce(),
        F3: FnOnce(),
        F4: FnOnce(),
        F5: FnOnce(),
    {
        // TODO: implement your dining philosophers solution here
        // You can translate your C++ logic into Rust inside this function.
        let mut mtx = self.mtx.lock().unwrap();
        while *mtx == 5 {
            mtx = self.cv.wait(mtx).unwrap();
        }
        *mtx += 1;
        drop(mtx);
        let left_fork = self.forks[(philosopher + 1) as usize % 5].lock().unwrap();
        pick_left_fork();
        let right_fork = self.forks[philosopher as usize].lock().unwrap();
        pick_right_fork();
        eat();
        put_left_fork();
        drop(left_fork);
        put_right_fork();
        drop(right_fork);
        let mut mtx = self.mtx.lock().unwrap();
        *mtx -= 1;
        self.cv.notify_all();
    }
}
unsafe impl Send for DiningPhilosophers {}
unsafe impl Sync for DiningPhilosophers {}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::{
        sync::{Arc, Mutex},
        thread,
        time::Duration,
    };
    #[test]
    fn foobar() {
        for it in [20] {
            let dining_philosophers = Arc::new(DiningPhilosophers::new());
            let shared_fork = vec![
                Arc::new(Mutex::new(1)),
                Arc::new(Mutex::new(1)),
                Arc::new(Mutex::new(1)),
                Arc::new(Mutex::new(1)),
                Arc::new(Mutex::new(1)),
            ];
            let mut handles = vec![];
            for i in 0..5 {
                let philosopher = Arc::clone(&dining_philosophers);
                let left_fork = Arc::clone(&shared_fork[(i + 1) % 5]);
                let right_fork = Arc::clone(&shared_fork[i]);
                let handle = std::thread::spawn(move || {
                    let mut rng = rand::thread_rng();
                    for _ in 0..it {
                        philosopher.wants_to_eat(
                            i as i32,
                            || {
                                let mut fork = left_fork.lock().unwrap();
                                assert_eq!(*fork, 1);
                                *fork = 0;
                            },
                            || {
                                let mut fork = right_fork.lock().unwrap();
                                assert_eq!(*fork, 1);
                                *fork = 0;
                            },
                            || thread::sleep(Duration::from_millis(rng.gen_range(200..1000))),
                            || {
                                let mut fork = left_fork.lock().unwrap();
                                assert_eq!(*fork, 0);
                                *fork = 1;
                            },
                            || {
                                let mut fork = right_fork.lock().unwrap();
                                assert_eq!(*fork, 0);
                                *fork = 1;
                            },
                        )
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        }
    }
}
