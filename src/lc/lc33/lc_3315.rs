// https://leetcode.com/problems/construct-the-minimum-bitwise-array-ii/
// 3315. Construct the Minimum Bitwise OR Array II
pub struct Solution;
impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        for n in nums {
            let n1 = n.trailing_ones();
            if n1 == 0 {
                ans.push(-1);
            } else {
                ans.push(n ^ (1 << (n1 - 1)));
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_bitwise_array() {
        assert_eq!(Solution::min_bitwise_array(vec![2, 3, 5, 7]), [-1, 1, 4, 3]);
        assert_eq!(Solution::min_bitwise_array(vec![11, 13, 31]), [9, 12, 15]);
    }
}
