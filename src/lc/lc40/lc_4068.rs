// https://leetcode.com/problems/maximize-meeting-earnings-with-idle-gaps/
// 4068. Maximize Meeting Earnings with Idle Gaps
pub struct Solution;
impl Solution {
    pub fn max_earnings(meetings: Vec<Vec<i32>>) -> i64 {
        let mut meetings = meetings;
        meetings.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut pre_max = vec![i64::MIN; meetings.len() + 1];
        let mut ans = 0;
        let e0 = meetings[0][1];
        for i in 0..meetings.len() {
            let start = meetings[i][0];
            let end = meetings[i][1];
            let earn = meetings[i][2];
            let mut maxi = earn as i64;
            if start >= e0 {
                let j = meetings[0..i].partition_point(|x| x[1] <= start);
                maxi += pre_max[j] + start as i64;
            }
            ans = ans.max(maxi);
            pre_max[i + 1] = pre_max[i].max(maxi - end as i64);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_earnings() {
        assert_eq!(Solution::max_earnings(vec_vec![[2, 5, 4], [6, 8, 3]]), 8);
        assert_eq!(Solution::max_earnings(vec_vec![[3, 5, 4], [4, 7, 8], [8, 10, 3]]), 12);
        assert_eq!(Solution::max_earnings(vec_vec![[1, 2, 2], [4, 5, 2], [7, 9, 3]]), 11);
    }
}
