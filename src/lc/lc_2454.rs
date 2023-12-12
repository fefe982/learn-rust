// https://leetcode.com/problems/next-greater-element-iv/
// 2454. Next Greater Element IV
pub struct Solution;
impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut s1 = vec![];
        let mut s2 = vec![];
        let mut ans = vec![-1; nums.len()];
        for (i, &n) in nums.iter().enumerate() {
            while s2.len() > 0 {
                if nums[*s2.last().unwrap()] < n {
                    ans[s2.pop().unwrap()] = n;
                } else {
                    break;
                }
            }
            let mut pos = s1.len();
            while pos > 0 {
                if nums[s1[pos - 1]] < n {
                    pos -= 1;
                } else {
                    break;
                }
            }
            let split = s1.split_at(pos);
            s2.append(&mut split.1.to_vec());
            s1.truncate(pos);
            s1.push(i);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_second_greater_element() {
        assert_eq!(
            Solution::second_greater_element(vec![2, 4, 0, 9, 6]),
            vec![9, 6, 6, -1, -1]
        );
        assert_eq!(Solution::second_greater_element(vec![3, 3]), vec![-1, -1]);
    }
}
