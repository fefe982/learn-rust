// https://leetcode.com/problems/count-the-number-of-houses-at-a-certain-distance-i/
// 3015. Count the Number of Houses at a Certain Distance i
pub struct Solution;
impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        for i in 1..n {
            for j in i + 1..=n {
                let d = (i - j)
                    .abs()
                    .min((i - x).abs() + 1 + (j - y).abs())
                    .min((i - y).abs() + 1 + (j - x).abs());
                ans[d as usize - 1] += 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_of_pairs() {
        assert_eq!(Solution::count_of_pairs(3, 1, 3), vec![6, 0, 0]);
        assert_eq!(Solution::count_of_pairs(5, 2, 4), vec![10, 8, 2, 0, 0]);
        assert_eq!(Solution::count_of_pairs(4, 1, 1), vec![6, 4, 2, 0]);
    }
}
