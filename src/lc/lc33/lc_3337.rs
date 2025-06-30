// https://leetcode.com/problems/total-characters-in-string-after-transformations-ii/
// 3337. Total Characters in String After Transformations II
pub struct Solution;
impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut cnt = [0; 26];
        for c in s.as_bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
        let mut pow = [[0; 26]; 26];
        for i in 0..26 {
            for j in 1..=nums[i] as usize {
                pow[i][(i + j) % 26] = 1;
            }
        }
        let mut t = t;
        let m = 1_000_000_007;
        while t > 0 {
            if t & 1 == 1 {
                let mut tmp = [0; 26];
                for i in 0..26 {
                    for j in 0..26 {
                        tmp[i] = (tmp[i] + cnt[j] * pow[j][i]) % m;
                    }
                }
                cnt = tmp;
            }
            let mut npow = [[0; 26]; 26];
            for i in 0..26 {
                for j in 0..26 {
                    for k in 0..26 {
                        npow[i][j] = (npow[i][j] + pow[i][k] * pow[k][j]) % m;
                    }
                }
            }
            pow = npow;
            t >>= 1;
        }
        (cnt.into_iter().sum::<i64>() % m) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_after_transformations() {
        assert_eq!(
            Solution::length_after_transformations(
                "abcyy".to_string(),
                2,
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
            ),
            7
        );
        assert_eq!(
            Solution::length_after_transformations(
                "azbk".to_string(),
                1,
                vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
            ),
            8
        );
    }
}
