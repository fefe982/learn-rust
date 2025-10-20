// https://leetcode.com/problems/the-kth-factor-of-n/
// 1492. The kth Factor of n
pub struct Solution;
impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut f = vec![];
        let mut c = 1;
        for i in 1..=n {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                if c == k {
                    return i;
                }
                c += 1;
                let d = n / i;
                if d != i {
                    f.push(d);
                }
            }
        }
        if c + f.len() as i32 - 1 < k {
            return -1;
        }
        f[f.len() - 1 - (k - c) as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kth_factor() {
        assert_eq!(Solution::kth_factor(24, 6), 8);
        assert_eq!(Solution::kth_factor(12, 3), 3);
        assert_eq!(Solution::kth_factor(7, 2), 7);
        assert_eq!(Solution::kth_factor(4, 4), -1);
    }
}
