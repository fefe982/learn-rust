// https://leetcode.com/problems/most-visited-sector-in-a-circular-track/
// 1560. Most Visited Sector in a Circular Track
pub struct Solution;
impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let e = rounds[rounds.len() - 1];
        let mut i = rounds[0];
        while i != e {
            ans.push(i);
            i += 1;
            if i > n {
                i = 1;
            }
        }
        ans.push(e);
        ans.sort();
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn most_visited() {
        assert_eq!(Solution::most_visited(4, vec![1, 3, 1, 2]), vec![1, 2]);
        assert_eq!(Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]), vec![2]);
        assert_eq!(Solution::most_visited(7, vec![1, 3, 5, 7]), vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
