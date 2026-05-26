// https://leetcode.com/problems/maximum-points-in-an-archery-competition/
// 2212. Maximum Points in an Archery Competition
pub struct Solution;
impl Solution {
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; 12];
        let mut max_points = 0;
        for mask in 0..(1 << 12) {
            let mut points = 0;
            let mut arrows = 0;
            for i in 0..12 {
                if (mask >> i) & 1 > 0 {
                    points += i;
                    arrows += alice_arrows[i] + 1;
                }
            }
            if arrows <= num_arrows && points > max_points {
                max_points = points;
                ans = vec![0; 12];
                for i in 0..12 {
                    if (mask >> i) & 1 > 0 {
                        ans[i] = alice_arrows[i] + 1;
                    }
                }
                if arrows < num_arrows {
                    ans[0] += num_arrows - arrows;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_bob_points() {
        assert_eq!(
            Solution::maximum_bob_points(9, vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0]),
            vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 2, 3, 1]
        );
        assert_eq!(
            Solution::maximum_bob_points(3, vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2]),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0]
        );
    }
}
