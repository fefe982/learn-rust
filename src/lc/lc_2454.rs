// https://leetcode.com/problems/next-greater-element-iv/
// 2454. Next Greater Element IV
pub struct Solution;
impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut s1 = vec![];
        let mut s2: Vec<usize> = vec![];
        let mut ans = vec![-1; nums.len()];
        for (i, &n) in nums.iter().enumerate() {
            while let Some(&i) = s2.last() {
                if nums[i] < n {
                    ans[i] = n;
                    s2.pop();
                } else {
                    break;
                }
            }
            let mut pos = s1.len();
            while pos > 0 && nums[s1[pos - 1]] < n {
                pos -= 1;
            }
            s2.extend(s1.iter().skip(pos));
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
