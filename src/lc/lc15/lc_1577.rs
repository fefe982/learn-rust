// https://leetcode.com/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/
// 1577. Number of Ways Where Square of Number Is Equal to Product of Two Numbers
pub struct Solution;
impl Solution {
    fn count(nums1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        for &num in nums1 {
            let num = num as i64;
            cnt.entry(num * num).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut c = 0;
        for i in 0..nums2.len() {
            for j in i + 1..nums2.len() {
                if let Some(&x) = cnt.get(&(nums2[i] as i64 * nums2[j] as i64)) {
                    c += x;
                }
            }
        }
        c
    }
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::count(&nums1, &nums2) + Self::count(&nums2, &nums1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_triplets() {
        assert_eq!(Solution::num_triplets(vec![43024, 99908], vec![1864]), 0);
        assert_eq!(Solution::num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
        assert_eq!(Solution::num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
        assert_eq!(Solution::num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]), 2);
    }
}
