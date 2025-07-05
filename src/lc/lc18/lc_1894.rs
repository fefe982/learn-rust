// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/
// 1894. Find the Student that Will Replace the Chalk
pub struct Solution;
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut chalk = chalk;
        if chalk[0] > k {
            return 0;
        }
        for i in 1..chalk.len() {
            chalk[i] += chalk[i - 1];
            if chalk[i] > k {
                return i as i32;
            }
        }
        let n = chalk.len();
        let k = k % chalk[n - 1];
        chalk.partition_point(|&x| x <= k) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chalk_replacer() {
        assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
        assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
    }
}
