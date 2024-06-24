// https://leetcode.com/problems/next-greater-element-ii/
// 503. Next Greater Element II
pub struct Solution;
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut s = vec![];
        let n = nums.len();
        let mut res = vec![-1; n];
        for (i, &num) in nums.iter().chain(nums.iter()).enumerate() {
            while let Some(&(li, ln)) = s.last() {
                if ln < num {
                    if li < n {
                        res[li] = num;
                    }
                    s.pop();
                } else {
                    break;
                }
            }
            s.push((i, num));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_greater_elements() {
        assert_eq!(Solution::next_greater_elements(vec![1, 2, 1]), [2, -1, 2]);
        assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 3]), [2, 3, 4, -1, 4]);
    }
}
