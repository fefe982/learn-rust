// https://leetcode.com/problems/finding-the-users-active-minutes/
// 1817. Finding the Users Active Minutes
pub struct Solution;
impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};

        let mut user_minutes: HashMap<i32, HashSet<i32>> = HashMap::new();
        for log in logs {
            let user_id = log[0];
            let minute = log[1];
            user_minutes.entry(user_id).or_default().insert(minute);
        }

        let mut answer = vec![0; k as usize];
        for minutes in user_minutes.values() {
            let uam = minutes.len();
            answer[uam - 1] += 1;
        }

        answer
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn finding_users_active_minutes() {
        assert_eq!(
            Solution::finding_users_active_minutes(vec_vec![[0, 5], [1, 2], [0, 2], [0, 5], [1, 3]], 5),
            [0, 2, 0, 0, 0]
        );
        assert_eq!(
            Solution::finding_users_active_minutes(vec_vec![[1, 1], [2, 2], [2, 3]], 4),
            [1, 1, 0, 0]
        );
    }
}
