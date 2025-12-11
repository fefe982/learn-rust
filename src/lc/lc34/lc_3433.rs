// https://leetcode.com/problems/count-mentions-per-user/
// 3433. Count Mentions per User
pub struct Solution;
impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let number_of_users = number_of_users as usize;
        let parseid = |s: &str| -> Vec<usize> {
            if s == "ALL" {
                (0..number_of_users).collect()
            } else if s == "HERE" {
                vec![]
            } else {
                s.split_whitespace().map(|x| x[2..].parse::<usize>().unwrap()).collect()
            }
        };
        let mut events = events
            .into_iter()
            .map(|event| {
                if event[0] == "MESSAGE" {
                    (event[1].parse().unwrap(), 1, parseid(&event[2]))
                } else {
                    (
                        event[1].parse::<i32>().unwrap(),
                        0,
                        vec![event[2].parse::<usize>().unwrap()],
                    )
                }
            })
            .collect::<Vec<_>>();
        events.sort_unstable();
        let mut ans = vec![0; number_of_users];
        let mut off = vec![-100; number_of_users];
        for event in events {
            if event.1 == 0 {
                off[event.2[0]] = event.0;
            } else {
                if event.2.is_empty() {
                    for i in 0..number_of_users {
                        if off[i] + 60 <= event.0 {
                            ans[i] += 1;
                        }
                    }
                } else {
                    for i in event.2 {
                        ans[i] += 1;
                    }
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
    fn count_mentions() {
        assert_eq!(
            Solution::count_mentions(
                2,
                vec_vec_str![
                    ["MESSAGE", "70", "HERE"],
                    ["OFFLINE", "10", "0"],
                    ["OFFLINE", "71", "0"]
                ]
            ),
            vec![1, 1]
        );
        assert_eq!(
            Solution::count_mentions(
                2,
                vec_vec_str![
                    ["MESSAGE", "10", "id1 id0"],
                    ["OFFLINE", "11", "0"],
                    ["MESSAGE", "71", "HERE"]
                ]
            ),
            vec![2, 2]
        );
        assert_eq!(
            Solution::count_mentions(
                2,
                vec_vec_str![
                    ["MESSAGE", "10", "id1 id0"],
                    ["OFFLINE", "11", "0"],
                    ["MESSAGE", "12", "ALL"]
                ]
            ),
            vec![2, 2]
        );
        assert_eq!(
            Solution::count_mentions(2, vec_vec_str![["OFFLINE", "10", "0"], ["MESSAGE", "12", "HERE"]]),
            vec![0, 1]
        );
    }
}
