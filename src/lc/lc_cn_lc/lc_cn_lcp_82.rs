// https://leetcode.cn/problems/cnHoX6/
// LCP 82. 万灵之树
pub struct Solution;
use std::cell::RefCell;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HashNode {
    x: i64,
    y: i32,
    t: i32,
}
struct Hash {
    h: Vec<HashNode>,
    t: i32,
}
impl Hash {
    fn new() -> Self {
        Self {
            h: vec![HashNode { x: 0, y: 0, t: 0 }; (1 << 16) + 105],
            t: 1,
        }
    }
    fn h(x: i64) -> usize {
        ((x as u32).wrapping_mul(1996090921) >> 16) as usize
    }

    fn insert(&mut self, x: i64) {
        let mut idx = Self::h(x) as usize;
        while self.h[idx].t == self.t {
            if self.h[idx].x == x {
                self.h[idx].y += 1;
                return;
            }
            idx += 1;
        }
        self.h[idx].x = x;
        self.h[idx].y = 1;
        self.h[idx].t = self.t;
    }

    fn find(&self, x: i64) -> i32 {
        let mut idx = Self::h(x) as usize;
        while self.h[idx].t == self.t {
            if self.h[idx].x == x {
                return self.h[idx].y;
            }
            idx += 1;
        }
        0
    }
    fn new_hash(&mut self) {
        self.t += 1;
    }
}

const N: i32 = 9;
const M0: usize = 205;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    v1: i64,
    v2: i64,
    l: usize,
}

fn log10(n: i32) -> i32 {
    if n < 10 {
        1
    } else {
        log10(n / 10) + 1
    }
}

fn inv(a: i64, m: i64) -> i64 {
    let mut a = a;
    let mut b = 1;
    while b % a != 0 {
        let t = m / a + 1;
        a = (a * t) % m;
        b = (b * t) % m;
    }
    b / a
}

fn fac(n: i32) -> i64 {
    let mut n = n as i64;
    let mut res: i64 = 1;
    while n > 0 {
        res *= n;
        n -= 1;
    }
    res
}

fn catalan(n: i32, m: i32) -> i64 {
    let mut res: i64 = 1;
    let n = n as i64;
    let m = m as i64;
    for i in 1..=m {
        res = res * (n - i + 1) / i;
    }
    res
}

struct Local {
    pow10: [i64; M0],
    pinv: [i64; M0],
    l: [usize; N as usize],
    len: [usize; 1 << N],
    n: i32,
    ans: i32,
    p: i64,
    r: i64,
    b: i32,
    c: Vec<RefCell<Vec<i64>>>,
    d: Vec<RefCell<Vec<Node>>>,
    hash: Hash,
}

impl Local {
    fn calc<F1, F2, F3>(&mut self, t0: i32, f1: F1, f2: F2, f3: F3)
    where
        F1: Fn(usize, usize) -> bool,
        F2: Fn(usize, usize) -> bool,
        F3: Fn(usize) -> bool,
    {
        for i in ((1 << self.n) + 1)..(1 << (self.n + 1)) {
            self.d[i].borrow_mut().clear()
        }

        for i in (1 << self.n)..(1 << (self.n + 1)) as usize {
            if i.count_ones() <= t0 as u32 {
                let mut j = (i - 1) & i;
                let mut _f2 = false;

                while j >> self.n != 0 {
                    _f2 = f2(i, j);
                    if _f2 || f1(i, j) {
                        for t in self.d[j].borrow().iter() {
                            let l1 = self.len[j - (1 << self.n)] - t.l + if j > (1 << self.n) { 2 } else { 0 };

                            for &v2 in self.c[i - j].borrow().iter() {
                                self.d[i].borrow_mut().push(Node {
                                    v1: (t.v1 + self.pow10[l1]) % self.p,

                                    v2: (t.v2 * self.pow10[self.len[i - j] + 1] + v2 * 10 + 9) % self.p,
                                    l: t.l + self.len[i - j] + 1,
                                });

                                if !_f2 {
                                    self.d[i].borrow_mut().push(Node {
                                        v1: (t.v1 + self.pow10[l1 + self.len[i - j]] + v2 * self.pow10[l1]) % self.p,

                                        v2: (t.v2 * 10 + 9) % self.p,
                                        l: t.l + 1,
                                    });
                                }
                            }
                        }
                    }
                    j = (j - 1) & i;
                }

                let j = (1 << (self.n + 1)) - 1 - i;

                if f3(j) {
                    continue;
                }
                self.hash.new_hash();
                for &v in (self.c)[j].borrow().iter() {
                    self.hash.insert(v);
                }

                for t in self.d[i].borrow().iter() {
                    self.ans += self.hash.find(
                        ((self.r - t.v2 - t.v1 * self.pow10[self.len[j] as usize + t.l as usize]) % self.p + self.p)
                            * self.pinv[t.l as usize]
                            % self.p,
                    );
                }
            }
        }
    }

    fn tree_of_infinite_souls(&mut self, a: Vec<i32>, p: i32, r: i32) -> i32 {
        self.n = a.len() as i32;
        self.ans = 0;
        self.b = (self.n + 2) / 3;
        self.p = p as i64;
        self.r = r as i64;

        if p == 2 || p == 5 {
            if p == 2 && r == 1 || p == 5 && r == 4 {
                return (catalan((self.n - 1) * 2, self.n - 1) / self.n as i64 * fac(self.n)) as i32;
            }
            return 0;
        }

        self.pow10[0] = 1 % self.p;
        for i in 1..M0 {
            self.pow10[i] = self.pow10[i - 1] as i64 * 10 % self.p;
        }

        for i in 0..M0 {
            self.pinv[i] = inv(self.pow10[i], self.p)
        }

        for i in 0..self.n as usize {
            self.l[i] = log10(a[i]) as usize
        }

        for i in 1..(1 << self.n) as usize {
            self.len[i] = (i.count_ones() as usize * 2 - 1) * 2;

            for j in 0..self.n as usize {
                if i & (1 << j) != 0 {
                    self.len[i] += self.l[j];
                }
            }
        }

        for i in 0..self.n as usize {
            self.c[1 << i]
                .borrow_mut()
                .push((a[i] as i64 * 10 + self.pow10[self.l[i] + 1] + 9) % self.p);
        }

        for i in 1..(1 << self.n) as usize {
            if i.count_ones() as i32 <= self.b * 2 {
                //component below u

                let t = self.pow10[self.len[i] - 1] + 9;

                let mut j = (i - 1) & i;
                //for (int j=(i-1)&i;j;j=(j-1)&i)
                while j != 0 {
                    if self.n == 9
                        || (i.count_ones() as i32) < ((self.n + 1) / 2).max(2)
                        || (j.count_ones().max((i - j).count_ones()) as i32) <= self.b.min((self.n - 1) / 2)
                    {
                        for &v1 in self.c[j].borrow().iter() {
                            let t1 = v1 * self.pow10[self.len[i - j] as usize + 1] + t;

                            for &v2 in self.c[i - j].borrow().iter() {
                                self.c[i].borrow_mut().push((v2 * 10 + t1) % self.p);
                            }
                        }
                    }
                    j = (j - 1) & i;
                }
            }
        }

        self.d[1 << self.n].borrow_mut().push(Node { v1: 0, v2: 0, l: 0 }); //component above u

        let yes = |_: usize, _: usize| -> bool { true };
        let no = |_: usize, _: usize| -> bool { false };
        if self.n == 9 {
            self.calc(4, yes, no, |j: usize| j.count_ones() != 6); //remove size 6
            let n = self.n;
            self.calc(
                5,
                |i: usize, j: usize| j != (1 << n) || (i - j).count_ones() >= 2, //remove size 5
                no,
                |j: usize| j.count_ones() != 5,
            );

            self.calc(
                6,
                |i: usize, j: usize| j != (1 << n) || (i - j).count_ones() == 3, //remove size 4
                |i: usize, j: usize| j == (1 << n) && (i - j).count_ones() == 4,
                |j: usize| j.count_ones() != 4,
            );
        } else {
            let n = self.n;
            let b = self.b;
            self.calc(
                n / 2 + 1,
                yes,
                |i: usize, j: usize| n % 2 == 0 && j.count_ones() == 1 && (i - j).count_ones() as i32 == n / 2,
                |j: usize| (j.count_ones() as i32) < (n + 1) / 2 || j.count_ones() as i32 > b * 2,
            );
        }
        return self.ans;
    }
}

impl Solution {
    pub fn tree_of_infinite_souls(gem: Vec<i32>, p: i32, target: i32) -> i32 {
        let local = &mut Local {
            pow10: [0; M0],
            pinv: [0; M0],
            l: [0; N as usize],
            len: [0; 1 << N],
            n: 0,
            ans: 0,
            p: 0,
            r: 0,
            b: 0,
            c: vec![RefCell::new(Vec::new()); 1 << N],
            d: vec![RefCell::new(Vec::new()); 1 << (N + 1)],
            hash: Hash::new(),
        };
        local.tree_of_infinite_souls(gem, p, target)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::tree_of_infinite_souls(vec![2, 3], 100000007, 11391299), 1);
        assert_eq!(Solution::tree_of_infinite_souls(vec![3, 21, 3], 7, 5,), 4);
    }
}
