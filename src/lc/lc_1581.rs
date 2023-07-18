// https://leetcode.cn/problems/minimum-interval-to-include-each-query/description/
// 1851. Minimum Interval to Include Each Query
pub struct Solution;
impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut res = vec![-1; queries.len()];
        intervals.sort_unstable();
        let mut qi = queries
            .into_iter()
            .zip(0usize..)
            .collect::<Vec<(i32, usize)>>();
        qi.sort_unstable();
        let mut i = 0;
        let mut que = std::collections::BinaryHeap::new();
        for (q, idx) in qi {
            while i < intervals.len() && intervals[i][0] <= q {
                que.push((
                    std::cmp::Reverse(intervals[i][1] - intervals[i][0] + 1),
                    intervals[i][1],
                ));
                i += 1;
            }
            while let Some(&(std::cmp::Reverse(len), end)) = que.peek() {
                if end >= q {
                    res[idx] = len;
                    break;
                } else {
                    que.pop();
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_interval() {
        assert_eq!(
            Solution::min_interval(vec_vec![[1, 4], [2, 4], [3, 6], [4, 4]], vec![2, 3, 4, 5]),
            vec![3, 3, 1, 4]
        );
        assert_eq!(
            Solution::min_interval(
                vec_vec![[2, 3], [2, 5], [1, 8], [20, 25]],
                vec![2, 19, 5, 22]
            ),
            vec![2, -1, 4, 6]
        );
    }
}
