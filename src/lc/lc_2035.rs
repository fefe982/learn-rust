// https://leetcode.com/problems/partition-array-into-two-arrays-to-minimize-sum-difference/
// 2035. Partition Array Into Two Arrays to Minimize Sum Difference
pub struct Solution;
impl Solution {
    fn get_sum(num: &[i32]) -> std::collections::HashMap<usize, std::collections::BTreeSet<i32>> {
        let mut sum_map = std::collections::HashMap::<usize, std::collections::BTreeSet<i32>>::new();
        let mut sum = vec![0; 1 << num.len()];
        for i in 1..1 << num.len() {
            sum[i] = sum[(i - 1) & i] + num[i.trailing_zeros() as usize] * 2;
            sum_map.entry(i.count_ones() as usize).or_default().insert(sum[i]);
        }
        sum_map.entry(0).or_default().insert(0);
        sum_map
    }
    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() / 2;
        let sum_map_left = Self::get_sum(&nums[0..n]);
        let sum_map_right = Self::get_sum(&nums[n..]);
        let mut ans = i32::MAX;
        let sum = sum_map_left.get(&n).unwrap().first().unwrap() + sum_map_right.get(&n).unwrap().first().unwrap();
        let half = sum / 2;
        for i in 0..=n {
            for &left in sum_map_left.get(&i).unwrap() {
                let right = half - left;
                if let Some(&r) = sum_map_right.get(&(n - i)).unwrap().range(right..).next() {
                    ans = ans.min(((left + r) * 2 - sum).abs() / 2);
                }
                if let Some(&r) = sum_map_right.get(&(n - i)).unwrap().range(..=right).next_back() {
                    ans = ans.min(((left + r) * 2 - sum).abs() / 2);
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_difference() {
        assert_eq!(Solution::minimum_difference(vec![3, 9, 7, 3]), 2);
        assert_eq!(Solution::minimum_difference(vec![-36, 36]), 72);
        assert_eq!(Solution::minimum_difference(vec![2, -1, 0, 4, -2, -9]), 0);
    }
}
