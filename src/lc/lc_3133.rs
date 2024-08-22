// https://leetcode.com/problems/minimum-array-end/
// 3133. Maximum Number of Moves in a Grid
pub struct Solution;
impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut n = n - 1;
        let mut powx = 1;
        let mut pown = 1;
        let mut res = 0;
        let x = x as i64;
        while n > 0 {
            if x & powx == 0 {
                if n & pown != 0 {
                    res += powx;
                    n &= !pown;
                }
                pown <<= 1;
            }
            powx <<= 1;
        }
        res | (x as i64)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_end() {
        assert_eq!(Solution::min_end(3, 4), 6);
        assert_eq!(Solution::min_end(2, 7), 15);
    }
}
