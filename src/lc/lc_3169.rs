// https://leetcode.com/problems/count-days-without-meetings/
// 3169. Count Days Without Meetings
pub struct Solution;
impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut day = 0;
        let mut meetings = meetings;
        meetings.sort();
        let mut start = -1;
        let mut end = -1;
        for meeting in meetings {
            if start == -1 {
                start = meeting[0];
                end = meeting[1];
            } else {
                if meeting[0] > end {
                    day += end - start + 1;
                    start = meeting[0];
                    end = meeting[1];
                } else {
                    end = end.max(meeting[1]);
                }
            }
        }
        if start != -1 {
            day += end - start + 1;
        }
        days - day
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_days() {
        assert_eq!(Solution::count_days(10, vec_vec![[5, 7], [1, 3], [9, 10]]), 2);
        assert_eq!(Solution::count_days(5, vec_vec![[2, 4], [1, 3]]), 1);
        assert_eq!(Solution::count_days(6, vec_vec![[1, 6]]), 0);
    }
}
