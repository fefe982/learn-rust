// https://leetcode.com/problems/delete-operation-for-two-strings/
// 583. Delete Operation for Two Strings
pub struct Solution;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut v = (0..=word1.len() as i32).collect::<Vec<_>>();
        for i in 0..word2.len() {
            let mut vv = v.clone();
            vv[0] = i as i32 + 1;
            for j in 0..word1.len() {
                if word1[j] == word2[i] {
                    vv[j + 1] = v[j];
                } else {
                    vv[j + 1] = vv[j].min(v[j + 1]) + 1;
                }
            }
            v = vv;
        }
        v[word1.len()]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_distance() {
        assert_eq!(Solution::min_distance("sea".to_string(), "eat".to_string()), 2);
        assert_eq!(Solution::min_distance("leetcode".to_string(), "etco".to_string()), 4);
    }
}
