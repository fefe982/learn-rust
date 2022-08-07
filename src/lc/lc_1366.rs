// https://leetcode.com/problems/rank-teams-by-votes/
// 1366. Rank Teams by Votes

pub struct Solution;
impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let mut cnt: Vec<Vec<i32>> = vec![vec![0; 28]; 26];
        for vote in votes {
            for (rank, team) in vote.chars().enumerate() {
                let t: usize = team as usize - 'A' as usize;
                cnt[t][0] = 1;
                cnt[t][rank + 1] += 1;
                cnt[t][27] = -(team as i32);
            }
        }
        cnt.sort_unstable();
        let mut res = String::from("");
        for c in cnt.iter().rev() {
            if c[0] == 0 {
                break;
            }
            res.push((-c[27]) as u8 as char)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rank_teams() {
        assert_eq!(
            Solution::rank_teams(vec![
                String::from("ABC"),
                String::from("ACB"),
                String::from("ABC"),
                String::from("ACB"),
                String::from("ACB")
            ]),
            "ACB"
        );
        assert_eq!(
            Solution::rank_teams(vec![String::from("WXYZ"), String::from("XYZW")]),
            String::from("XWYZ")
        );
        assert_eq!(
            Solution::rank_teams(vec![String::from("ZMNAGUEDSJYLBOPHRQICWFXTVK")]),
            String::from("ZMNAGUEDSJYLBOPHRQICWFXTVK")
        );
    }
}
