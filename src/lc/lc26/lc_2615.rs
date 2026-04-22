// https://leetcode.com/problems/sum-of-distances/
// 2615. Sum of Distances
pub struct Solution;
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut ans = vec![0; nums.len()];
        let mut groups = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            groups.entry(num).or_insert_with(Vec::new).push(i as i64);
        }
        for indices in groups.values() {
            let mut prefix_sum = 0;
            for (i, &index) in indices.iter().enumerate() {
                ans[index as usize] += index * (i as i64) - prefix_sum;
                prefix_sum += index;
            }
            let mut suffix_sum: i64 = 0;
            for (i, &index) in indices.iter().rev().enumerate() {
                ans[index as usize] += suffix_sum - index * (i as i64);
                suffix_sum += index;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance() {
        assert_eq!(Solution::distance(vec![1, 3, 1, 1, 2]), vec![5, 0, 3, 4, 0]);
        assert_eq!(Solution::distance(vec![0, 5, 3]), vec![0, 0, 0]);
    }
}
