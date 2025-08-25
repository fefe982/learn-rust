// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-ii/
// 3027. Find the Number of Ways to Place People II
pub struct Solution;
impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|v| (v[0], std::cmp::Reverse(v[1])));
        let mut res = 0;
        for i in 0..points.len() {
            let x0 = points[i][0];
            let y0 = points[i][1];
            let mut ylim = i32::MIN;
            let mut cx = x0;
            let mut found_cx = false;
            for j in i + 1..points.len() {
                let x1 = points[j][0];
                let y1 = points[j][1];
                if x1 > cx {
                    cx = x1;
                    found_cx = false;
                }
                if found_cx || y1 <= ylim || y1 > y0 {
                    continue;
                }
                ylim = y1;
                res += 1;
                found_cx = true;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_of_ways() {
        assert_eq!(Solution::number_of_pairs(vec_vec![[1, 1], [2, 2], [3, 3]]), 0);
        assert_eq!(Solution::number_of_pairs(vec_vec![[6, 2], [4, 4], [2, 6]]), 2);
        assert_eq!(Solution::number_of_pairs(vec_vec![[3, 1], [1, 3], [1, 1]]), 2);
    }
}
