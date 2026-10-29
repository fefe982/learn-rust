// https://leetcode.com/problems/partition-array-into-k-distinct-groups/
// 3659. Partition Array Into K-Distinct Groups
pub struct Solution;
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        if nums.len() % k != 0 {
            return false;
        }
        let mut cnt = std::collections::HashMap::new();
        let cc = nums.len() / k;
        for num in nums {
            let c = cnt.entry(num).or_insert(0);
            *c += 1;
            if *c > cc {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition_array() {
        assert_eq!(Solution::partition_array(vec![1, 2, 3, 4], 2), true);
        assert_eq!(Solution::partition_array(vec![3, 5, 2, 2], 2), true);
        assert_eq!(Solution::partition_array(vec![1, 5, 2, 3], 3), false);
    }
}
