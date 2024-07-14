// https://leetcode.com/problems/booking-concert-tickets-in-groups/
// 2286. Booking Concert Tickets in Groups
#[derive(Debug)]
pub struct BookMyShow {
    max_empty: Vec<(i32, i64)>,
    end: Vec<i32>,
    n: i32,
    m: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {
    pub fn new(n: i32, m: i32) -> Self {
        let mut nn = 1;
        while nn < n {
            nn <<= 1;
        }
        nn <<= 1;
        let mut s = Self {
            max_empty: vec![(0, 0); nn as usize],
            end: vec![0; n as usize],
            n,
            m,
        };
        for i in 0..n {
            s.setv(0, 0, nn / 2, i, m);
        }
        s
    }
    fn setv(&mut self, idx: usize, l: i32, r: i32, i: i32, v: i32) {
        if l + 1 == r {
            self.max_empty[idx] = (v, v as i64);
            return;
        }
        let mid = (l + r) >> 1;
        if i < mid {
            self.setv((idx << 1) + 1, l, mid, i, v);
        } else {
            self.setv((idx << 1) + 2, mid, r, i, v);
        }
        self.max_empty[idx] = (
            self.max_empty[(idx << 1) + 1].0.max(self.max_empty[(idx << 1) + 2].0),
            self.max_empty[(idx << 1) + 1].1 + self.max_empty[(idx << 1) + 2].1,
        );
    }
    fn getn(&mut self, idx: usize, l: i32, r: i32, v: i32) -> usize {
        if self.max_empty[idx].0 < v {
            return usize::MAX;
        }
        if l + 1 == r {
            return l as usize;
        }
        let mid = (l + r) >> 1;
        if self.max_empty[(idx << 1) + 1].0 >= v {
            self.getn((idx << 1) + 1, l, mid, v)
        } else {
            self.getn((idx << 1) + 2, mid, r, v)
        }
    }
    fn getn_sum(&mut self, idx: usize, l: i32, r: i32, v: i64) -> usize {
        if self.max_empty[idx].1 < v {
            return usize::MAX;
        }
        if l + 1 == r {
            return l as usize;
        }
        let mid = (l + r) >> 1;
        if self.max_empty[(idx << 1) + 1].1 >= v {
            self.getn_sum((idx << 1) + 1, l, mid, v)
        } else {
            self.getn_sum((idx << 1) + 2, mid, r, v - self.max_empty[(idx << 1) + 1].1)
        }
    }
    fn add(&mut self, i: usize, v: i32) {
        self.end[i] += v;
        let left = self.m - self.end[i];
        self.setv(0, 0, self.max_empty.len() as i32 / 2, i as i32, left);
    }
    pub fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let idx = self.getn(0, 0, self.max_empty.len() as i32 / 2, k);
        if idx > max_row as usize {
            return vec![];
        }
        self.add(idx, k);
        vec![idx as i32, self.end[idx] - k]
    }

    pub fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let idx = self.getn_sum(0, 0, self.max_empty.len() as i32 / 2, k as i64);
        if idx > max_row as usize {
            return false;
        }
        let mut k = k;
        for i in 0..self.n as usize {
            if self.end[i] < self.m {
                let left = self.m - self.end[i];
                if left >= k {
                    self.add(i, k);
                    break;
                } else {
                    k -= left;
                    self.add(i, left);
                }
            }
        }
        true
    }
}

/**
 * Your BookMyShow object will be instantiated and called as such:
 * let obj = BookMyShow::new(n, m);
 * let ret_1: Vec<i32> = obj.gather(k, maxRow);
 * let ret_2: bool = obj.scatter(k, maxRow);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        let null = ();
        for case in [(
            vec!["BookMyShow", "gather", "gather", "scatter", "scatter"],
            vec_vec![[2, 5], [4, 0], [2, 0], [5, 1], [5, 1]],
            vec_any![null, [0, 0], [], true, false],
        )] {
            let mut obj = None;
            for ((&func, args), ret) in case.0.iter().zip(case.1.iter()).zip(case.2.iter()) {
                match func {
                    "BookMyShow" => obj = Some(BookMyShow::new(args[0], args[1])),
                    "gather" => assert_eq!(obj.as_mut().unwrap().gather(args[0], args[1]), ret.as_vec_i32()),
                    "scatter" => assert_eq!(obj.as_mut().unwrap().scatter(args[0], args[1]), ret.as_bool()),
                    _ => panic!(),
                }
            }
        }
    }
}
