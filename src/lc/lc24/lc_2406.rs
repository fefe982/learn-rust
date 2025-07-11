// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/
// 2406. Divide Intervals Into Minimum Number of Groups
pub struct Solution;
impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable();
        let mut end = std::collections::BinaryHeap::new();
        let mut ans = 1;
        for interval in intervals {
            while let Some(&std::cmp::Reverse(e)) = end.peek() {
                if e < interval[0] {
                    end.pop();
                } else {
                    break;
                }
            }
            ans = ans.max(end.len() as i32 + 1);
            end.push(std::cmp::Reverse(interval[1]));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_groups() {
        assert_eq!(
            Solution::min_groups(vec_vec![[5, 10], [6, 8], [1, 5], [2, 3], [1, 10]]),
            3
        );
        assert_eq!(Solution::min_groups(vec_vec![[1, 3], [5, 6], [8, 10], [11, 13]]), 1);
    }
}
