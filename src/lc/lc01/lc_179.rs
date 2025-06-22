// https://leetcode.com/problems/largest-number/
// 179. Largest Number
pub struct Solution;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        nums.sort_by(|a, b| {
            let mut a = a.clone();
            let mut b = b.clone();
            a.push_str(&b);
            b.push_str(&a);
            b.cmp(&a)
        });
        if nums[0].starts_with('0') {
            return String::from("0");
        }
        nums.join("")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_number() {
        assert_eq!(Solution::largest_number(vec![10, 2]), String::from("210"));
        assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), String::from("9534330"));
    }
}
