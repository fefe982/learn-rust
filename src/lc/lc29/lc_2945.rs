// https://leetcode.com/problems/find-maximum-non-decreasing-array-length/
// 2945. Find the Maximum Non-decreasing Subarray Length
pub struct Solution;
impl Solution {
    pub fn find_maximum_length(nums: Vec<i32>) -> i32 {
        let mut stk = vec![(0, 0, 0)]; // (expect sum, current sum, current length)
        let mut sum = 0;
        let mut last = 0;
        for n in nums {
            sum += n as i64;
            while last + 1 < stk.len() && stk[last + 1].0 <= sum {
                last += 1;
            }
            let (_, last_sum, last_len) = stk[last];
            let next_expect = sum * 2 - last_sum;
            while let Some((execpt, _, _)) = stk.last() {
                if *execpt >= next_expect {
                    stk.pop();
                } else {
                    break;
                }
            }
            stk.push((next_expect, sum, last_len + 1));
        }
        stk.last().unwrap().2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_maximum_length() {
        assert_eq!(Solution::find_maximum_length(vec![5, 2, 2]), 1);
        assert_eq!(Solution::find_maximum_length(vec![1, 2, 3, 4]), 4);
        assert_eq!(Solution::find_maximum_length(vec![4, 3, 2, 6]), 3);
    }
}
