// https://leetcode.com/problems/minimum-number-of-operations-to-make-x-and-y-equal/
// 2998. Minimum Number of Operations to Make X and Y Equal
pub struct Solution;
impl Solution {
    fn solve(x: i32, y: i32, steps: &mut Vec<i32>) -> i32 {
        if x == y {
            return 0;
        }
        if steps[x as usize] != -1 {
            return steps[x as usize];
        }
        let mut ans = (x - y).abs();
        if x > y {
            ans = ans.min(Self::solve(x / 5, y, steps) + 1 + x % 5);
            if x % 5 != 0 {
                ans = ans.min(Self::solve(x / 5 + 1, y, steps) + 6 - x % 5);
            }
            ans = ans.min(Self::solve(x / 11, y, steps) + 1 + x % 11);
            if x % 11 != 0 {
                ans = ans.min(Self::solve(x / 11 + 1, y, steps) + 12 - x % 11);
            }
        }
        steps[x as usize] = ans;
        ans
    }
    pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
        let mut steps = vec![-1; 10001];
        Self::solve(x, y, &mut steps)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_operations_to_make_equal() {
        assert_eq!(Solution::minimum_operations_to_make_equal(26, 1), 3);
        assert_eq!(Solution::minimum_operations_to_make_equal(54, 2), 4);
        assert_eq!(Solution::minimum_operations_to_make_equal(25, 30), 5);
    }
}
