// https://leetcode.com/problems/gray-code/
// 89. Gray Code
pub struct Solution;
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(1 << n);
        let mut v = 0;
        ans.push(v);
        for i in 1..(1 << n) as i32 {
            v ^= 1 << (i.trailing_zeros() as i32);
            ans.push(v);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: i32) {
        let ans = Solution::gray_code(n);
        let l = (1 << n) as usize;
        assert!(ans.len() == l);
        let mut h = std::collections::HashSet::new();
        for i in 0..l {
            h.insert(ans[i]);
            assert!(ans[i] >= 0 && ans[i] < l as i32);
            assert!((ans[i] ^ ans[(i + 1) % l]).count_ones() == 1);
        }
        assert!(h.len() == l);
    }
    #[test]
    fn test_gray_code() {
        check(2);
        check(1);
    }
}
