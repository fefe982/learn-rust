// https://leetcode.com/problems/count-no-zero-pairs-that-sum-to-n/
// 3704. Count No-Zero Integers
pub struct Solution;
impl Solution {
    fn count(n: &Vec<i32>, i: usize, borrow: usize, num: usize, cache: &mut Vec<Vec<Vec<i64>>>) -> i64 {
        if i == n.len() {
            return 1 - borrow as i64;
        }
        if cache[i][borrow][num] != -1 {
            return cache[i][borrow][num];
        }
        let mut d = n[i];
        d -= borrow as i32;
        let mut ans = 0;
        if num == 1 {
            if d > 1 {
                ans += (d - 1) as i64 * Self::count(n, i + 1, 0, 1, cache);
            }
            if d >= 0 && d < 9 {
                ans += (9 - d) as i64 * Self::count(n, i + 1, 1, 1, cache);
            }
            if d == -1 {
                ans += 8 * Self::count(n, i + 1, 1, 1, cache);
            }
        }
        if i > 0 {
            if d != 0 {
                ans += Self::count(n, i + 1, if d < 0 { 1 } else { 0 }, 0, cache) * (num + 1) as i64;
            } else if i == n.len() - 1 {
                ans += 1;
            }
        }
        cache[i][borrow][num] = ans;
        ans
    }
    pub fn count_no_zero_pairs(n: i64) -> i64 {
        let mut v = vec![];
        let mut n = n;
        while n > 0 {
            v.push((n % 10) as i32);
            n /= 10;
        }
        println!("{:?}", v);
        Self::count(&v, 0, 0, 1, &mut vec![vec![vec![-1; 2]; 2]; v.len()])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_no_zero_pairs() {
        assert_eq!(Solution::count_no_zero_pairs(100), 90);
        assert_eq!(Solution::count_no_zero_pairs(2), 1);
        assert_eq!(Solution::count_no_zero_pairs(3), 2);
        assert_eq!(Solution::count_no_zero_pairs(11), 8);
    }
}
