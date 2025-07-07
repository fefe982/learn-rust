// https://leetcode.com/problems/maximum-number-of-balls-in-a-box/
// 1742. Maximum Number of Balls in a Box
pub struct Solution;
impl Solution {
    fn cnt(mut n: i32) -> Vec<i32> {
        let v = [
            [
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0,
            ],
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0,
            ],
            [
                1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 63, 69, 73, 75, 75, 73, 69, 63, 55, 45, 36, 28, 21, 15, 10, 6, 3,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            [
                1, 4, 10, 20, 35, 56, 84, 120, 165, 220, 282, 348, 415, 480, 540, 592, 633, 660, 670, 660, 633, 592,
                540, 480, 415, 348, 282, 220, 165, 120, 84, 56, 35, 20, 10, 4, 1,
            ],
        ];
        let mut vr = vec![0; 46];
        let mut sum = 0;
        let mut m = 10000;
        for c in (0..4).rev() {
            let d = n / m;
            n %= m;
            for d in 0..d as usize {
                for i in 0..37 {
                    if v[c][i] == 0 {
                        break;
                    }
                    vr[i + d + sum as usize] += v[c][i];
                }
            }
            sum += d;
            m /= 10;
        }
        for i in 0..=n as usize {
            vr[i + sum as usize] += 1;
        }
        vr
    }
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let vh = Solution::cnt(high_limit);
        let vl = Solution::cnt(low_limit - 1);
        vh.into_iter().zip(vl.into_iter()).map(|(a, b)| a - b).max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_balls() {
        assert_eq!(Solution::count_balls(6745, 28696), 1499);
        assert_eq!(Solution::count_balls(5515, 9973), 340);
        assert_eq!(Solution::count_balls(1, 10), 2);
        assert_eq!(Solution::count_balls(5, 15), 2);
        assert_eq!(Solution::count_balls(19, 28), 2);
    }
}
