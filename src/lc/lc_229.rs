// https://leetcode.com/problems/majority-element-ii/
// 229. Majority Element II
pub struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut cand1 = 0;
        let mut cand2 = 0;
        let mut c1 = 0;
        let mut c2 = 0;
        for &num in nums.iter() {
            if num == cand1 {
                c1 += 1;
            } else if num == cand2 {
                c2 += 1;
            } else if c1 == 0 {
                cand1 = num;
                c1 = 1;
            } else if c2 == 0 {
                cand2 = num;
                c2 = 1;
            } else {
                c1 -= 1;
                c2 -= 1;
            }
        }
        c1 = 0;
        c2 = 0;
        let mut res = vec![];
        for &num in nums.iter() {
            if num == cand1 {
                c1 += 1;
            } else if num == cand2 {
                c2 += 1;
            }
        }
        if c1 > nums.len() / 3 {
            res.push(cand1);
        }
        if c2 > nums.len() / 3 {
            res.push(cand2);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_majority_element_ii() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq!(Solution::majority_element(vec![1]), vec![1]);
        assert_eq!(Solution::majority_element(vec![1, 2]), vec![1, 2]);
    }
}
