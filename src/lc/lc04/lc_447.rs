// https://leetcode.com/problems/number-of-boomerangs/
// 447. Number of Boomerangs
pub struct Solution;
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![std::collections::HashMap::<i32, i32>::new(); points.len()];
        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let dx = points[i][0] - points[j][0];
                let dy = points[i][1] - points[j][1];
                let d = dx * dx + dy * dy;
                *count[i].entry(d).or_default() += 1;
                *count[j].entry(d).or_default() += 1;
            }
        }
        let mut c = 0;
        for m in count {
            for (_, cnt) in m {
                c += cnt * (cnt - 1);
            }
        }
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_of_boomerangs() {
        assert_eq!(Solution::number_of_boomerangs(vec_vec![[0, 0], [1, 0], [2, 0]]), 2);
        assert_eq!(Solution::number_of_boomerangs(vec_vec![[1, 1], [2, 2], [3, 3]]), 2);
        assert_eq!(Solution::number_of_boomerangs(vec_vec![[1, 1]]), 0);
    }
}
