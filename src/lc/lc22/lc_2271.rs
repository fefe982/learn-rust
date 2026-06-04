// https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/
// 2271. Maximum White Tiles Covered by a Carpet
pub struct Solution;
impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut tiles = tiles;
        tiles.sort();
        let mut ans = 0;
        let mut sum = 0;
        let mut j = 0;
        for i in 0..tiles.len() {
            while j < tiles.len() && tiles[j][1] < tiles[i][0] + carpet_len {
                sum += tiles[j][1] - tiles[j][0] + 1;
                j += 1;
            }
            let mut half = 0;
            if j < tiles.len() && tiles[j][0] <= tiles[i][0] + carpet_len {
                half = tiles[i][0] + carpet_len - tiles[j][0]
            }
            ans = ans.max(sum + half);
            sum -= tiles[i][1] - tiles[i][0] + 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_white_tiles() {
        assert_eq!(
            Solution::maximum_white_tiles(vec_vec![[1, 5], [10, 11], [12, 18], [20, 25], [30, 32]], 10),
            9
        );
        assert_eq!(Solution::maximum_white_tiles(vec_vec![[10, 11], [1, 1]], 2), 2);
    }
}
