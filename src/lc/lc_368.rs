// https://leetcode.com/problems/largest-divisible-subset/
// 368. Largest Divisible Subset
pub struct Solution;
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut cnt = vec![1; nums.len()];
        let mut parent = vec![usize::MAX; nums.len()];
        let mut max = 0;
        let mut maxi = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && cnt[i] < cnt[j] + 1 {
                    cnt[i] = cnt[j] + 1;
                    parent[i] = j;
                    if cnt[i] > max {
                        max = cnt[i];
                        maxi = i;
                    }
                }
            }
        }
        let mut res = vec![];
        while maxi != usize::MAX {
            res.push(nums[maxi]);
            maxi = parent[maxi];
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    fn check(nums: Vec<i32>, size: usize) -> bool {
        let ans = Solution::largest_divisible_subset(nums.clone());
        if ans.len() != size {
            return false;
        }
        let nums = nums.into_iter().collect::<std::collections::HashSet<_>>();
        let mut last = 1;
        ans.into_iter()
            .sorted()
            .filter(|&x| {
                if !nums.contains(&x) {
                    return true;
                }
                if x % last != 0 {
                    return true;
                }
                last = x;
                false
            })
            .next()
            .is_none()
    }
    #[test]
    fn test_largest_divisible_subset() {
        check(vec![1, 2, 3], 2);
        check(vec![1, 2, 4, 8], 4);
    }
}
