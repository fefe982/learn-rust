// https://leetcode.com/problems/tuple-with-same-product/description/
// 1726. Tuple with Same Product
pub struct Solution;
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::<i32, i32>::new();
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                *m.entry(nums[i] * nums[j]).or_default() += 1;
            }
        }
        let mut sum = 0;
        for (_, v) in m {
            sum += v * (v - 1) / 2;
        }
        sum * 8
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tuple_same_product() {
        assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6]), 8);
        assert_eq!(Solution::tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
    }
}
