// https://leetcode.com/problems/counting-bits/
// 338. Counting Bits
pub struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut v = Vec::with_capacity(n + 1);
        v.push(0);
        let mut s = 1;
        while s <= n {
            for i in s..(s * 2).min(n + 1) {
                v.push(v[i - s] + 1);
            }
            s *= 2;
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_bits() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
