// https://leetcode.com/problems/maximum-points-inside-the-square/
// 3143. Maximum Points Inside the Square
pub struct Solution;
impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut p = points
            .into_iter()
            .map(|v| (v[0].abs().max(v[1].abs())))
            .zip(s.chars().map(|c| (c as u8 - b'a') as usize))
            .collect::<Vec<_>>();
        p.sort_unstable();
        let mut ans = 0;
        let mut cnt = vec![0; 26];
        let mut dist = 0;
        for (i, (d, c)) in p.into_iter().chain([(i32::MAX, 0)]).enumerate() {
            if d > dist {
                dist = d;
                ans = i as i32;
            }
            if cnt[c] > 0 {
                return ans;
            }
            cnt[c] += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_points_inside_square() {
        assert_eq!(
            Solution::max_points_inside_square(
                vec_vec![[16, 32], [27, 3], [23, -14], [-32, -16], [-3, 26], [-14, 33]],
                "aaabfc".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::max_points_inside_square(
                vec_vec![[2, 2], [-1, -2], [-4, 4], [-3, 1], [3, -3]],
                "abdca".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::max_points_inside_square(vec_vec![[1, 1], [-2, -2], [-2, 2]], "abb".to_string()),
            1
        );
        assert_eq!(
            Solution::max_points_inside_square(vec_vec![[1, 1], [-1, -1], [2, -2]], "ccd".to_string()),
            0
        );
    }
}
