// https://leetcode.com/problems/maximum-length-of-subarray-with-positive-product/
// 1567. Maximum Length of Subarray With Positive Product
pub struct Solution;
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut pos = 0;
        let mut neg = usize::MAX;
        let mut sgn = 1;
        let mut ans = 0;
        for i in 0..nums.len() {
            match nums[i].cmp(&0) {
                std::cmp::Ordering::Equal => {
                    pos = i + 1;
                    neg = usize::MAX;
                    sgn = 1;
                }
                std::cmp::Ordering::Greater => {
                    if sgn == 1 {
                        ans = ans.max(i - pos + 1);
                    } else {
                        ans = ans.max(i - neg + 1);
                    }
                }
                std::cmp::Ordering::Less => {
                    sgn = -sgn;
                    if sgn == 1 {
                        ans = ans.max(i - pos + 1);
                    } else {
                        if neg == usize::MAX {
                            neg = i + 1;
                        } else {
                            ans = ans.max(i - neg + 1);
                        }
                    }
                }
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_max_len() {
        assert_eq!(Solution::get_max_len(vec![1, -2, -3, 4]), 4);
        assert_eq!(Solution::get_max_len(vec![0, 1, -2, -3, -4]), 3);
        assert_eq!(Solution::get_max_len(vec![-1, -2, -3, 0, 1]), 2);
    }
}
