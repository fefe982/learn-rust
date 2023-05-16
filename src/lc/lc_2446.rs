// https://leetcode.com/problems/determine-if-two-events-have-conflict/
// 2446. Determine if Two Events Have Conflict
pub struct Solution;
impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        if event1[1] < event2[0] || event1[0] > event2[1] {
            false
        } else {
            true
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn have_conflict() {
        assert_eq!(
            Solution::have_conflict(vec_str!["01:15", "02:00"], vec_str!["02:00", "03:00"]),
            true
        );
        assert_eq!(
            Solution::have_conflict(vec_str!["01:00", "02:00"], vec_str!["01:20", "03:00"]),
            true
        );
        assert_eq!(
            Solution::have_conflict(vec_str!["10:00", "11:00"], vec_str!["14:00", "15:00"]),
            false
        );
    }
}
