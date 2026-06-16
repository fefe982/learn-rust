// https://leetcode.com/problems/query-kth-smallest-trimmed-number/
// 2343. Query Kth Smallest Trimmed Number
pub struct Solution;
impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; queries.len()];
        let mut query_idx = (0..queries.len()).collect::<Vec<_>>();
        query_idx.sort_unstable_by(|&i, &j| {
            let a = &queries[i][1];
            let b = &queries[j][1];
            a.cmp(b).then(i.cmp(&j))
        });
        let mut last_trim = 0;
        let mut idx = (0..nums.len()).collect::<Vec<_>>();
        for i in query_idx {
            let query = &queries[i];
            let k = query[0] as usize;
            let trim = query[1] as usize;
            if trim != last_trim {
                last_trim = trim;
                idx.sort_unstable_by(|&i, &j| {
                    let a = &nums[i][nums[i].len() - trim..];
                    let b = &nums[j][nums[j].len() - trim..];
                    a.cmp(b).then(i.cmp(&j))
                });
            }
            ans[i] = idx[k - 1] as i32;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_smallest_trimmed_numbers() {
        assert_eq!(
            Solution::smallest_trimmed_numbers(
                vec_str!["102", "473", "251", "814"],
                vec_vec![[1, 1], [2, 3], [4, 2], [1, 2]]
            ),
            vec![2, 2, 1, 0]
        );
        assert_eq!(
            Solution::smallest_trimmed_numbers(vec_str!["24", "37", "96", "04"], vec_vec![[2, 1], [2, 2]]),
            vec![3, 0]
        );
    }
}
