// https://leetcode.com/problems/substring-with-concatenation-of-all-words/
// 30. Substring with Concatenation of All Words
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let w_len = words[0].len();
        let w_cnt = words.len();
        let mut word_map = HashMap::new();
        for word in words.iter() {
            let w = word.as_bytes();
            if let Some(cnt) = word_map.get_mut(w) {
                *cnt += 1;
            } else {
                word_map.insert(w, 1);
            };
        }
        let mut visited = vec![false; s.len() - w_len * w_cnt + 1];
        let mut pos = Vec::new();
        let s = s.as_bytes();
        let full_len = w_len * w_cnt;
        for idx in 0..(s.len() - full_len + 1) {
            if visited[idx] {
                continue;
            }
            let sub_word = &s[idx..(idx + w_len)];
            if let Some(_) = word_map.get(sub_word) {
                let mut word_map = word_map.clone();
                *word_map.get_mut(sub_word).unwrap() -= 1;
                let mut check_start = idx;
                let mut check_end = idx + w_len;
                visited[check_start] = true;
                loop {
                    if check_start + full_len == check_end {
                        pos.push(check_start as i32);
                        *word_map
                            .get_mut(&s[check_start..(check_start + w_len)])
                            .unwrap() += 1;
                        check_start += w_len;
                        if check_start + full_len <= s.len() {
                            visited[check_start] = true;
                        } else {
                            break;
                        }
                    } else if check_end + w_len > s.len() {
                        break;
                    } else if let Some(cnt) = word_map.get_mut(&s[check_end..(check_end + w_len)]) {
                        if *cnt > 0 {
                            *cnt -= 1;
                            check_end += w_len;
                        } else if check_start + w_len + full_len > s.len() {
                            break;
                        } else {
                            let end_word = &s[check_end..(check_end + w_len)];
                            while check_start < check_end
                                && check_start + w_len + full_len <= s.len()
                            {
                                let start_word = &s[check_start..(check_start + w_len)];
                                *word_map.get_mut(&start_word).unwrap() += 1;
                                check_start += w_len;
                                visited[check_start] = true;
                                if start_word == end_word {
                                    break;
                                }
                            }
                        }
                    } else {
                        while check_start < check_end && check_start + w_len + full_len <= s.len() {
                            check_start += w_len;
                            visited[check_start] = true;
                        }
                        break;
                    }
                }
            }
        }
        pos
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_substring() {
        assert_eq!(
            Solution::find_substring(
                String::from("barfoothefoobarman"),
                vec![String::from("foo"), String::from("bar")]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("word")
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                String::from("barfoofoobarthefoobarman"),
                vec![
                    String::from("bar"),
                    String::from("foo"),
                    String::from("the"),
                ]
            ),
            vec![6, 9, 12]
        );
    }
}
