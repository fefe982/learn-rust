// https://leetcode.com/problems/neither-minimum-nor-maximum/
// 2733. Neither Minimum nor Maximum
pub struct Solution;
impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &x in nums.iter() {
            min = min.min(x);
            max = max.max(x);
        }
        for &x in nums.iter() {
            if x != min && x != max {
                return x;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_non_min_or_max() {
        fn test(nums: Vec<i32>) {
            let ans = Solution::find_non_min_or_max(nums.clone());
            let min = *nums.iter().min().unwrap();
            let max = *nums.iter().max().unwrap();
            let set: std::collections::HashSet<_> = nums.into_iter().filter(|&x| x != min && x != max).collect();
            assert!((set.is_empty() && ans == -1) || set.contains(&ans))
        }
        test(vec![3, 2, 1, 4]);
        test(vec![1, 2]);
        test(vec![2, 1, 3]);
    }
}
