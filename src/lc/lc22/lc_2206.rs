// https://leetcode.com/problems/divide-array-into-equal-pairs/
// 2206. Divide Array Into Equal Pairs
pub struct Solution;
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut v = [0; 501];
        let mut c = 0;
        for n in nums {
            if v[n as usize] % 2 == 0 {
                c += 1;
            } else {
                c -= 1;
            }
            v[n as usize] += 1;
        }
        c == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn divide_array() {
        assert_eq!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]), true);
        assert_eq!(Solution::divide_array(vec![1, 2, 3, 4]), false);
    }
}
