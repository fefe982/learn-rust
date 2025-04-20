// https://leetcode.com/problems/total-hamming-distance/
// 477. Total Hamming Distance
pub struct Solution;
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut c = [[0; 2]; 32];
        for mut i in nums {
            for j in 0..30 {
                c[j][(i & 1) as usize] += 1;
                i >>= 1;
            }
        }
        let mut res = 0;
        for j in 0..30 {
            res += c[j][0] * c[j][1];
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_hamming_distance() {
        assert_eq!(Solution::total_hamming_distance(vec![4, 14, 2]), 6);
        assert_eq!(Solution::total_hamming_distance(vec![4, 14, 4]), 4);
    }
}
