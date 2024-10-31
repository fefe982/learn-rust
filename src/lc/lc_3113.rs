// https://leetcode.com/problems/find-the-number-of-subarrays-where-boundary-elements-are-maximum/
// 3113. Find the Number of Subarrays Where Boundary Elements Are Maximum
pub struct Solution;
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
        let mut stk = vec![];
        let mut res = 0;
        for n in nums {
            let mut cnt = 1;
            while let Some(&(top, c)) = stk.last() {
                if n > top {
                    stk.pop();
                } else if n == top {
                    stk.pop();
                    cnt = c + 1;
                    break;
                } else {
                    break;
                }
            }
            res += cnt as i64;
            stk.push((n, cnt));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_subarrays() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 4, 3, 3, 2]), 6);
        assert_eq!(Solution::number_of_subarrays(vec![3, 3, 3]), 6);
    }
}
