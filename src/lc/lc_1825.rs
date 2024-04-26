// https://leetcode.com/problems/finding-mk-average/
// 1825. Finding MK Average
#[derive(Debug)]
struct MSet {
    m: std::collections::BTreeMap<i32, i32>,
    sz: usize,
}
impl MSet {
    fn new() -> Self {
        Self {
            m: std::collections::BTreeMap::new(),
            sz: 0,
        }
    }
    fn add(&mut self, num: i32) {
        *self.m.entry(num).or_default() += 1;
        self.sz += 1;
    }
    fn len(&self) -> usize {
        self.sz
    }
    fn remove(&mut self, num: i32) {
        if let Some(v) = self.m.get_mut(&num) {
            *v -= 1;
            if *v == 0 {
                self.m.remove(&num);
            }
            self.sz -= 1;
        }
    }
    fn min_element(&self) -> Option<i32> {
        self.m.iter().next().map(|(k, _v)| *k)
    }
    fn max_element(&self) -> Option<i32> {
        self.m.iter().next_back().map(|(k, _v)| *k)
    }
}
#[derive(Debug)]
pub struct MKAverage {
    m: usize,
    k: usize,
    q: std::collections::VecDeque<i32>,
    mmax: MSet,
    mmid: MSet,
    mmin: MSet,
    sum: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {
    pub fn new(m: i32, k: i32) -> Self {
        Self {
            m: m as usize,
            k: k as usize,
            q: std::collections::VecDeque::new(),
            mmax: MSet::new(),
            mmid: MSet::new(),
            mmin: MSet::new(),
            sum: 0,
        }
    }

    pub fn add_element(&mut self, num: i32) {
        self.q.push_back(num);
        if self.mmax.len() < self.k || self.mmax.min_element().unwrap() < num {
            self.mmax.add(num);
        } else if self.mmid.len() < self.m - self.k * 2 || self.mmid.min_element().unwrap() < num {
            self.mmid.add(num);
            self.sum += num;
        } else {
            self.mmin.add(num);
        }
        if self.q.len() > self.m {
            let n = self.q.pop_front().unwrap();
            if n >= self.mmax.min_element().unwrap() {
                self.mmax.remove(n);
            } else if n >= self.mmid.min_element().unwrap() {
                self.mmid.remove(n);
                self.sum -= n;
            } else {
                self.mmin.remove(n);
            }
        }
        if self.q.len() >= self.m {
            while self.mmax.len() > self.k {
                let n = self.mmax.min_element().unwrap();
                self.mmax.remove(n);
                self.mmid.add(n);
                self.sum += n;
            }
            while self.mmin.len() > self.k {
                let n = self.mmin.max_element().unwrap();
                self.mmin.remove(n);
                self.mmid.add(n);
                self.sum += n;
            }
            while self.mmax.len() < self.k {
                let n = self.mmid.max_element().unwrap();
                self.mmid.remove(n);
                self.mmax.add(n);
                self.sum -= n;
            }
            while self.mmin.len() < self.k {
                let n = self.mmid.min_element().unwrap();
                self.mmid.remove(n);
                self.mmin.add(n);
                self.sum -= n;
            }
        }
    }

    pub fn calculate_mk_average(&self) -> i32 {
        if self.q.len() < self.m {
            -1
        } else {
            self.sum / (self.m - self.k * 2) as i32
        }
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use itertools;
    #[test]
    fn test_mk_average() {
        let null = 0;
        let mut obj = MKAverage::new(0, 0);
        for (op, param, result) in itertools::multizip((
            vec_str![
                "MKAverage",
                "addElement",
                "addElement",
                "calculateMKAverage",
                "addElement",
                "calculateMKAverage",
                "addElement",
                "addElement",
                "addElement",
                "calculateMKAverage"
            ],
            vec_vec![[3, 1], [3], [1], [], [10], [], [5], [5], [5], []],
            vec![null, null, null, -1, null, 3, null, null, null, 5],
        )) {
            match op.as_str() {
                "MKAverage" => obj = MKAverage::new(param[0], param[1]),
                "addElement" => obj.add_element(param[0]),
                "calculateMKAverage" => assert_eq!(obj.calculate_mk_average(), result),
                _ => panic!("invalid op"),
            }
        }
    }
}
