// https://leetcode.cn/problems/WHnhjV/
// LCP 50. 宝石补给
pub struct Solution;
impl Solution {
    pub fn give_gem(gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        let mut gem = gem;
        for op in operations {
            let (x, y) = (op[0] as usize, op[1] as usize);
            let offer = gem[x] / 2;
            gem[x] -= offer;
            gem[y] += offer;
        }
        let mut min = i32::MAX;
        let mut max = 0;
        for g in gem {
            min = min.min(g);
            max = max.max(g);
        }
        max - min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_give_gem() {
        assert_eq!(
            Solution::give_gem(vec![3, 1, 2], vec_vec![[0, 2], [2, 1], [2, 0]]),
            2
        );
        assert_eq!(
            Solution::give_gem(
                vec![100, 0, 50, 100],
                vec_vec![[0, 2], [0, 1], [3, 0], [3, 0]]
            ),
            75
        );
        assert_eq!(
            Solution::give_gem(vec![0, 0, 0, 0], vec_vec![[1, 2], [3, 1], [1, 2]]),
            0
        );
    }
}
