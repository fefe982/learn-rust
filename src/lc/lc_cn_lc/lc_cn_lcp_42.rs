// https://leetcode.cn/problems/vFjcfV/
// LCP 42. 玩具套圈
pub struct Solution;
impl Solution {
    pub fn circle_game(toys: Vec<Vec<i32>>, circles: Vec<Vec<i32>>, r: i32) -> i32 {
        let set = circles
            .into_iter()
            .map(|x| (x[0], x[1]))
            .collect::<std::collections::HashSet<_>>();
        let mut cnt = 0;
        'toy: for toy in toys {
            let tx = toy[0];
            let ty = toy[1];
            let tr = toy[2];
            let dr = r - tr;
            let dr2 = dr as i64 * dr as i64;

            for dx in -dr..=dr {
                let dx2 = dx as i64 * dx as i64;
                for dy in -dr..=dr {
                    if dx2 + dy as i64 * dy as i64 <= dr2 && set.contains(&(tx + dx, ty + dy)) {
                        cnt += 1;
                        continue 'toy;
                    }
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::circle_game(vec_vec![[3, 3, 1], [3, 2, 1]], vec_vec![[4, 3]], 2),
            1
        );
        assert_eq!(
            Solution::circle_game(vec_vec![[1, 3, 2], [4, 3, 1], [7, 1, 2]], vec_vec![[1, 0], [3, 3]], 4),
            2
        );
    }
}
