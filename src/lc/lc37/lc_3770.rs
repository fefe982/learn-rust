// https://leetcode.com/problems/largest-prime-from-consecutive-prime-sum/
// 3770. Largest Prime From Consecutive Prime Sum
pub struct Solution;
impl Solution {
    pub fn largest_prime(n: i32) -> i32 {
        let n = n as usize;
        let mut v = vec![true; n + 1];
        let mut sum = 0;
        let mut sv = vec![];
        for i in 2..=n {
            if v[i] {
                if sum < n {
                    sum += i;
                    sv.push(sum);
                }
                for j in (i * i..=n).step_by(i) {
                    v[j] = false;
                }
            }
            if i * i > n && sum >= n {
                break;
            }
        }
        for s in sv.into_iter().rev() {
            if s <= n && v[s] {
                return s as i32;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_prime() {
        assert_eq!(Solution::largest_prime(20), 17);
        assert_eq!(Solution::largest_prime(2), 2);
    }
}
