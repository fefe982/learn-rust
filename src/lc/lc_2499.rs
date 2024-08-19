// https://leetcode.com/problems/minimum-total-cost-to-make-arrays-unequal/
// 2499. Minimum Total Cost to Make Arrays Unequal
pub struct Solution;
impl Solution {
    pub fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut map1 = std::collections::HashMap::<i32, i32>::new();
        let mut map2 = std::collections::HashMap::new();
        let mut map_dup = std::collections::HashMap::<i32, i32>::new();
        let len = nums1.len();
        let mut ans = 0;
        let mut dup_cnt = 0;
        let mut max_dup_cnt = 0;
        let mut max_dup_val = 0;
        for (i, (&n1, &n2)) in nums1.iter().zip(nums2.iter()).enumerate() {
            if n1 == n2 {
                let dupc = map_dup.entry(n1).or_default();
                *dupc += 1;
                ans += i as i64;
                if *dupc > max_dup_cnt {
                    max_dup_cnt = *dupc;
                    max_dup_val = n1;
                }
                dup_cnt += 1;
            }
            *map1.entry(n1).or_default() += 1;
            *map2.entry(n2).or_default() += 1;
        }
        for (n, c) in map1 {
            if map2.get(&n).unwrap_or(&0) + c > len as i32 {
                return -1;
            }
        }
        let mut i = 0;
        for _ in 0..(max_dup_cnt * 2 - dup_cnt).max(0) {
            while nums1[i] == nums2[i] || nums1[i] == max_dup_val || nums2[i] == max_dup_val {
                i += 1;
            }
            ans += i as i64;
            i += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_total_cost() {
        assert_eq!(
            Solution::minimum_total_cost(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            10
        );
        assert_eq!(
            Solution::minimum_total_cost(vec![2, 2, 2, 1, 3], vec![1, 2, 2, 3, 3]),
            10
        );
        assert_eq!(Solution::minimum_total_cost(vec![1, 2, 2], vec![1, 2, 2]), -1);
    }
}
