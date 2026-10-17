// https://leetcode.com/problems/minimum-adjacent-swaps-to-alternate-parity/
// 3587. Minimum Adjacent Swaps to Alternate Parity
pub struct Solution;
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut e = vec![];
        let mut o = vec![];
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                e.push(i);
            } else {
                o.push(i);
            }
        }
        if ((e.len() as i32) - (o.len() as i32)).abs() > 1 {
            return -1;
        }
        let mut m = i32::MAX;
        if e.len() >= o.len() {
            let mut s = 0;
            for i in 0..e.len() {
                s += ((i as i32 * 2) - e[i] as i32).abs();
            }
            m = m.min(s);
        }
        if o.len() >= e.len() {
            let mut s = 0;
            for i in 0..o.len() {
                s += ((i as i32 * 2) - o[i] as i32).abs();
            }
            m = m.min(s);
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_swaps() {
        assert_eq!(Solution::min_swaps(vec![2, 4, 6, 5, 7]), 3);
        assert_eq!(Solution::min_swaps(vec![2, 4, 5, 7]), 1);
        assert_eq!(Solution::min_swaps(vec![1, 2, 3]), 0);
        assert_eq!(Solution::min_swaps(vec![4, 5, 6, 8]), -1);
    }
}
