// https://leetcode.com/problems/brick-wall/
// 554. Brick Wall
pub struct Solution;
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut min = wall.len() as i32;
        let total = min;
        let mut h = std::collections::BinaryHeap::new();
        for row in wall {
            let mut pos = 0;
            for i in 0..row.len() - 1 {
                pos += row[i];
                h.push(pos);
            }
        }
        let mut last_pos = 0;
        let mut count = 0;
        while let Some(pos) = h.pop() {
            if pos != last_pos {
                min = min.min(total - count);
                count = 1;
                last_pos = pos;
            } else {
                count += 1;
            }
        }
        min.min(total - count)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn least_bricks() {
        assert_eq!(
            Solution::least_bricks(vec_vec![
                [1, 2, 2, 1],
                [3, 1, 2],
                [1, 3, 2],
                [2, 4],
                [3, 1, 2],
                [1, 3, 1, 1]
            ]),
            2
        );
        assert_eq!(Solution::least_bricks(vec_vec![[1], [1], [1]]), 3);
    }
}
