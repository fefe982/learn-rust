// https://leetcode.com/problems/shortest-common-supersequence/
// 1092. Shortest Common Supersequence
pub struct Solution;
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];
        for j in 0..str2.len() {
            dp[0][j + 1] = j + 1;
        }
        for i in 0..str1.len() {
            dp[i + 1][0] = i + 1;
            for j in 0..str2.len() {
                if str1[i] == str2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = std::cmp::min(dp[i][j + 1], dp[i + 1][j]) + 1;
                }
            }
        }
        let mut res = vec![0u8; dp[str1.len()][str2.len()]];
        let mut idx1 = str1.len();
        let mut idx2 = str2.len();
        let mut idx = dp[str1.len()][str2.len()];
        while idx > 0 {
            idx -= 1;
            if idx1 == 0 {
                res[idx] = str2[idx2 - 1];
                idx2 -= 1;
            } else if idx2 == 0 {
                res[idx] = str1[idx1 - 1];
                idx1 -= 1;
            } else if str1[idx1 - 1] == str2[idx2 - 1] {
                res[idx] = str1[idx1 - 1];
                idx1 -= 1;
                idx2 -= 1;
            } else {
                if dp[idx1][idx2] == dp[idx1 - 1][idx2] + 1 {
                    res[idx] = str1[idx1 - 1];
                    idx1 -= 1;
                } else {
                    res[idx] = str2[idx2 - 1];
                    idx2 -= 1;
                }
            }
        }
        String::from_utf8(res).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortest_common_supersequence() {
        assert_eq!(
            Solution::shortest_common_supersequence(String::from("abac"), String::from("cab")),
            String::from("cabac")
        );
        assert_eq!(
            Solution::shortest_common_supersequence(
                String::from("aaaaaaaa"),
                String::from("aaaaaaaa")
            ),
            String::from("aaaaaaaa")
        );
    }
}
