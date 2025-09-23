// https://leetcode.com/problems/get-watched-videos-by-your-friends/
// 1311. Get Watched Videos by Your Friends
pub struct Solution;
impl Solution {
    pub fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        level: i32,
    ) -> Vec<String> {
        let id = id as usize;
        let mut friend = vec![];
        if level == 0 {
            friend.push(id);
        } else {
            let mut visited = vec![false; friends.len()];
            visited[id] = true;
            let mut q = std::collections::VecDeque::new();
            q.push_back((0, id));
            while let Some((l, i)) = q.pop_front() {
                if l == level {
                    friend.push(i);
                } else {
                    for &f in &friends[i] {
                        if !visited[f as usize] {
                            visited[f as usize] = true;
                            q.push_back((l + 1, f as usize));
                        }
                    }
                }
            }
        }
        let mut cnt = std::collections::HashMap::new();
        for &i in &friend {
            for v in &watched_videos[i] {
                *cnt.entry(v).or_insert(0) += 1;
            }
        }
        let mut ans = cnt.into_iter().map(|(s, i)| (i, s)).collect::<Vec<_>>();
        ans.sort();
        ans.into_iter().map(|(_, s)| s.clone()).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn watched_videos_by_friends() {
        assert_eq!(
            Solution::watched_videos_by_friends(
                vec_vec_str![["A", "B"], ["C"], ["B", "C"], ["D"]],
                vec_vec![[1, 2], [0, 3], [0, 3], [1, 2]],
                0,
                1
            ),
            vec_str!["B", "C"]
        );
        assert_eq!(
            Solution::watched_videos_by_friends(
                vec_vec_str![["A", "B"], ["C"], ["B", "C"], ["D"]],
                vec_vec![[1, 2], [0, 3], [0, 3], [1, 2]],
                0,
                2
            ),
            vec_str!["D"]
        );
    }
}
