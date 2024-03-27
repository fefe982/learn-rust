// https://leetcode.com/problems/count-ways-to-group-overlapping-ranges/
// 2580. Count Ways to Group Overlapping Ranges
pub struct Solution;
impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let mut ranges = ranges;
        ranges.sort();
        let mut cnt = 1;
        let mut end = ranges[0][1];
        for i in 1..ranges.len() {
            if ranges[i][0] <= end {
                end = end.max(ranges[i][1]);
            } else {
                cnt += 1;
                end = ranges[i][1];
            }
        }
        let mut ans = 1;
        let mut pow = 2;
        while cnt > 0 {
            if cnt % 2 == 1 {
                ans = (ans * pow) % 1000000007i64;
            }
            pow = (pow * pow) % 1_0000_0000_7;
            cnt /= 2;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_ways() {
        assert_eq!(Solution::count_ways(vec_vec![[0, 0], [8, 9], [12, 13], [1, 3]]), 16);
        assert_eq!(Solution::count_ways(vec_vec![[6, 10], [5, 15]]), 2);
        assert_eq!(Solution::count_ways(vec_vec![[1, 3], [10, 20], [2, 5], [4, 8]]), 4);
    }
}
