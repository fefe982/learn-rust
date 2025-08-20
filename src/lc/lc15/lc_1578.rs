// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/
// 1578. Minimum Time to Make Rope Colorful
pub struct Solution;
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut last_sum = 0;
        let mut last_color = ' ';
        let mut last_cost = 0;
        for (c, &t) in colors.chars().zip(needed_time.iter()) {
            if c != last_color {
                sum += last_sum - last_cost;
                last_color = c;
                last_sum = t;
                last_cost = t;
            } else {
                last_sum += t;
                last_cost = last_cost.max(t);
            }
        }
        sum + (last_sum - last_cost)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_cost() {
        assert_eq!(Solution::min_cost(String::from("abaac"), vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(Solution::min_cost(String::from("abc"), vec![1, 2, 3]), 0);
        assert_eq!(Solution::min_cost(String::from("aabaa"), vec![1, 2, 3, 4, 1]), 2);
    }
}
