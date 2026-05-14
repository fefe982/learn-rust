// https://leetcode.com/problems/intervals-between-identical-elements/
// 2121. Intervals Between Identical Elements
pub struct Solution;
impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        let mut ans = vec![0; arr.len()];
        let mut idx = std::collections::HashMap::<i32, Vec<usize>>::new();
        for (i, &x) in arr.iter().enumerate() {
            idx.entry(x).or_default().push(i);
        }
        for v in idx.values() {
            let m = v.len();
            if m <= 1 {
                continue;
            }
            let mut prefix_sum = vec![0; m + 1];
            for i in 0..m {
                prefix_sum[i + 1] = prefix_sum[i] + v[i] as i64;
            }
            for i in 0..m {
                let left = i as i64 * v[i] as i64 - prefix_sum[i];
                let right = prefix_sum[m] - prefix_sum[i + 1] - (m as i64 - 1 - i as i64) * v[i] as i64;
                ans[v[i]] = right + left;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_distances() {
        assert_eq!(
            Solution::get_distances(vec![2, 1, 3, 1, 2, 3, 3]),
            vec![4, 2, 7, 2, 4, 4, 5]
        );
        assert_eq!(Solution::get_distances(vec![10, 5, 10, 10]), vec![5, 0, 3, 4]);
    }
}
