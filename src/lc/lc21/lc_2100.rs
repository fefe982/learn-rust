// https://leetcode.com/problems/find-good-days-to-rob-the-bank/
// 2100. Find Good Days to Rob the Bank
pub struct Solution;
impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        if time * 2 >= security.len() as i32 {
            return Vec::new();
        }
        let n = security.len();
        let mut pre = vec![0; n];
        let mut post = vec![0; n];
        for i in 1..n {
            if security[i] <= security[i - 1] {
                pre[i] = pre[i - 1] + 1;
            }
        }
        for i in (0..n - 1).rev() {
            if security[i] <= security[i + 1] {
                post[i] = post[i + 1] + 1;
            }
        }
        let mut ans = Vec::new();
        for i in time as usize..n - time as usize {
            if pre[i] >= time && post[i] >= time {
                ans.push(i as i32);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_good_days_to_rob_bank() {
        assert_eq!(
            Solution::good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2),
            vec![2, 3]
        );
        assert_eq!(
            Solution::good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(Solution::good_days_to_rob_bank(vec![1, 2, 3, 4, 5, 6], 2), vec![]);
    }
}
