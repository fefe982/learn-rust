// https://leetcode.cn/problems/find-the-lexicographically-largest-string-from-the-box-i/
// 3403. Find the Lexicographically Largest String From the Box I
pub struct Solution;
impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let w = word.as_bytes();
        let mut i = 0;
        let mut j = 1;
        while j < w.len() {
            let mut l = 0;
            while j + l < w.len() && w[j + l] == w[i + l] {
                l += 1;
            }
            if j + l == w.len() {
                break;
            }
            if w[j + l] > w[i + l] {
                let d = i;
                i = j;
                j = (j + 1).max(d + l + 1);
            } else {
                j = j + l + 1;
            }
        }
        let l = (w.len() + 1 - num_friends as usize).min(w.len() - i);
        String::from_utf8(w[i..i + l].to_vec()).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn answer_string() {
        assert_eq!(Solution::answer_string("dbca".to_string(), 2), "dbc");
        assert_eq!(Solution::answer_string("gggg".to_string(), 4), "g");
    }
}
