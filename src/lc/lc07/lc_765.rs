// https://leetcode.com/problems/couples-holding-hands/
// 765. Couples Holding Hands
pub struct Solution;
impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let mut row = row;
        let mut res = 0;
        let mut pos = vec![0; row.len()];
        for (p, &r) in row.iter().enumerate() {
            pos[r as usize] = p;
        }
        for i in 0..row.len() / 2 {
            let mut position = i * 2;
            let mut cnt = 0;
            while row[position] >= 0 {
                let people = row[position];
                row[position] = -1;
                row[position ^ 1] = -1;
                cnt += 1;
                position = pos[(people as usize) ^ 1] ^ 1;
            }
            if cnt > 0 {
                res += cnt - 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_swaps_couples() {
        assert_eq!(Solution::min_swaps_couples(vec![5, 4, 2, 6, 3, 1, 0, 7]), 2);
        assert_eq!(Solution::min_swaps_couples(vec![0, 2, 1, 3]), 1);
        assert_eq!(Solution::min_swaps_couples(vec![3, 2, 0, 1]), 0);
    }
}
