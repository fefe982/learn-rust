// https://leetcode.com/problems/sum-of-imbalance-numbers-of-all-subarrays/
// 2763. Sum of Imbalance Numbers of All Subarrays
pub struct Solution;
impl Solution {
    pub fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut set = vec![false; nums.len() + 2];
            let mut imb = -1;
            for j in i..nums.len() {
                let nj = nums[j] as usize;
                if !set[nj] {
                    imb += 1;
                    if set[nj - 1] {
                        imb -= 1;
                    }
                    if set[nj + 1] {
                        imb -= 1;
                    }
                }
                set[nj] = true;
                ans += imb;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_imbalance_numbers() {
        assert_eq!(Solution::sum_imbalance_numbers(vec![2, 3, 1, 4]), 3);
        assert_eq!(Solution::sum_imbalance_numbers(vec![1, 3, 3, 3, 5]), 8);
    }
}
