// https://leetcode.com/problems/add-minimum-number-of-rungs/
// 1936. Add Minimum Number of Rungs
pub struct Solution;
impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        let mut ans = 0;
        let mut prev = 0;
        for rung in rungs {
            let gap = rung - prev;
            if gap > dist {
                ans += (gap - 1) / dist;
            }
            prev = rung;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_rungs() {
        assert_eq!(Solution::add_rungs(vec![1, 3, 5, 10], 2), 2);
        assert_eq!(Solution::add_rungs(vec![3, 6, 8, 10], 3), 0);
        assert_eq!(Solution::add_rungs(vec![3, 4, 6, 7], 2), 1);
    }
}
