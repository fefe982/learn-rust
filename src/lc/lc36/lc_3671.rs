// https://leetcode.com/problems/sum-of-beautiful-subsequences/
// 3671. Sum of Beautiful Subsequences
pub struct Solution;
impl Solution {
    pub fn total_beauty(nums: Vec<i32>) -> i32 {
        const M: i64 = 1_000_000_007;
        let mx = nums.iter().max().unwrap().clone() as usize;
        let mut g = vec![0; mx + 1];
        let mut divs = vec![vec![]; mx + 1];
        for i in 1..=mx / 2 {
            let mut j = i;
            while j <= mx {
                divs[j].push(i);
                j += i;
            }
        }
        let mut gp = vec![vec![]; mx / 2 + 1];
        for &n in &nums {
            for &i in &divs[n as usize] {
                gp[i].push(n);
            }
        }
        fn add(c: &mut Vec<i64>, mut i: usize, v: i64, len: usize) {
            while i < len {
                c[i] = (c[i] + v) % M;
                i += i & (!i + 1);
            }
        }
        fn get(c: &Vec<i64>, mut i: usize) -> i64 {
            let mut r = 0;
            while i > 0 {
                r = (r + c[i]) % M;
                i -= i & (!i + 1);
            }
            r
        }
        for i in 1..=mx / 2 {
            if gp[i].is_empty() {
                continue;
            }
            let len = mx / i + 1;
            let mut c = vec![0; len];
            for &n in &gp[i] {
                let n = n as usize;
                let ni = n / i;
                let sum = get(&c, ni - 1);
                add(&mut c, ni, sum + 1, len);
            }
            g[i] = get(&c, mx / i);
        }
        let hmx = (mx / 2) as i32;
        for &n in &nums {
            if n > hmx {
                g[n as usize] += 1;
            }
        }
        let mut res = 0;
        for i in (1..=mx).rev() {
            if g[i] == 0 {
                continue;
            }
            let mut j = i + i;
            while j <= mx {
                g[i] = (g[i] - g[j] + M) % M;
                j += i;
            }
            res = (res + i as i64 * g[i]) % M
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_beauty() {
        assert_eq!(Solution::total_beauty(vec![1, 2, 3]), 10);
        assert_eq!(Solution::total_beauty(vec![4, 6]), 12);
    }
}
