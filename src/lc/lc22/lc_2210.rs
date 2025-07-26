// https://leetcode.com/problems/count-hills-and-valleys-in-an-array/
// 2210. Count Hills and Valleys in an Array
pub struct Solution;
impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut last = 0;
        for i in 1..nums.len() {
            match nums[i].cmp(&nums[i - 1]) {
                std::cmp::Ordering::Greater => {
                    if last < 0 {
                        ans += 1;
                    }
                    last = 1;
                }
                std::cmp::Ordering::Less => {
                    if last > 0 {
                        ans += 1;
                    }
                    last = -1;
                }
                _ => {}
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_hill_valley() {
        assert_eq!(Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
        assert_eq!(Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
    }
}
