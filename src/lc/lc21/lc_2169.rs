// https://leetcode.com/problems/count-operations-to-obtain-zero/
// 2169. Count Operations to Obtain Zero
pub struct Solution;
impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let mut num1 = num1;
        let mut num2 = num2;
        let mut ans = 0;
        loop {
            if num1 == 0 {
                return ans;
            }
            ans += num2 / num1;
            num2 = num2 % num1;
            if num2 == 0 {
                return ans;
            }
            ans += num1 / num2;
            num1 = num1 % num2;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn count_operations() {
        assert_eq!(Solution::count_operations(2, 3), 3);
        assert_eq!(Solution::count_operations(10, 10), 1);
    }
}
