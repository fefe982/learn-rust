// https://leetcode.com/problems/closest-equal-element-queries/
// 3488. Closest Equal Element Queries
pub struct Solution;
impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        fn circular_dist(a: usize, b: usize, n: usize) -> i32 {
            let d = a.abs_diff(b);
            d.min(n - d) as i32
        }

        let n = nums.len();
        let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            pos.entry(v).or_default().push(i);
        }

        let mut best = vec![-1; n];
        for indices in pos.values() {
            let m = indices.len();
            if m <= 1 {
                continue;
            }
            for k in 0..m {
                let cur = indices[k];
                let prev = indices[(k + m - 1) % m];
                let next = indices[(k + 1) % m];
                best[cur] = circular_dist(cur, prev, n).min(circular_dist(cur, next, n));
            }
        }

        queries.into_iter().map(|i| best[i as usize]).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_queries() {
        assert_eq!(
            Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]),
            vec![2, -1, 3]
        );
        assert_eq!(
            Solution::solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3]),
            vec![-1, -1, -1, -1]
        );
    }
}
