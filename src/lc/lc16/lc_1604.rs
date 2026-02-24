// https://leetcode.com/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period/
// 1604. Alert Using Same Key-Card Three or More Times in a One Hour Period
pub struct Solution;
impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut pair = key_name
            .into_iter()
            .zip(
                key_time
                    .into_iter()
                    .map(|s| s.split(':').fold(0, |acc, x| acc * 60 + x.parse::<i32>().unwrap())),
            )
            .collect::<Vec<_>>();
        pair.sort();
        let mut ans = vec![];
        let mut s = 0;
        for (i, (name, time)) in pair.iter().enumerate() {
            if i == 0 || pair[i - 1].0 != *name {
                s = i;
            }
            while time - pair[s].1 > 60 {
                s += 1;
            }
            if i - s >= 2 {
                if let Some(e) = ans.last() {
                    if e != name {
                        ans.push(name.clone());
                    }
                } else {
                    ans.push(name.clone());
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn alert_names() {
        assert_eq!(
            Solution::alert_names(
                vec_str!["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"],
                vec_str!["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"]
            ),
            ["daniel"]
        );
        assert_eq!(
            Solution::alert_names(
                vec_str!["alice", "alice", "alice", "bob", "bob", "bob", "bob"],
                vec_str!["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"]
            ),
            ["bob"]
        );
    }
}
