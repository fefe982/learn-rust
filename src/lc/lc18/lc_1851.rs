// https://leetcode.com/problems/minimum-interval-to-include-each-query/
// 1851. Minimum Interval to Include Each Query
pub struct Solution;
impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut intervals = intervals;
        intervals.sort_by_key(|x| x[0]);
        let mut queries = queries.into_iter().zip(0usize..).collect::<Vec<_>>();
        queries.sort_unstable();
        let mut ans = vec![-1; queries.len()];
        let mut ql = std::collections::BinaryHeap::new();
        let mut i = 0;
        for (q, idx) in queries {
            while i < intervals.len() && intervals[i][0] <= q {
                ql.push(std::cmp::Reverse((intervals[i][1] - intervals[i][0], intervals[i][1])));
                i += 1;
            }
            while let Some(&std::cmp::Reverse((len, r))) = ql.peek() {
                if r < q {
                    ql.pop();
                } else {
                    ans[idx] = len + 1;
                    break;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_interval() {
        assert_eq!(
            Solution::min_interval(vec_vec![[1, 4], [2, 4], [3, 6], [4, 4]], vec![2, 3, 4, 5]),
            [3, 3, 1, 4]
        );
        assert_eq!(
            Solution::min_interval(vec_vec![[2, 3], [2, 5], [1, 8], [20, 25]], vec![2, 19, 5, 22]),
            [2, -1, 4, 6]
        );
    }
}
