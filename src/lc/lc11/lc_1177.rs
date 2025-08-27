// https://leetcode.com/problems/can-make-palindrome-from-substring/
// 1177. Can Make Palindrome from Substring
pub struct Solution;
impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut pos = vec![Vec::new(); 26];
        for (i, &c) in s.as_bytes().iter().enumerate() {
            pos[(c - b'a') as usize].push(i as i32);
        }

        queries
            .into_iter()
            .map(|q| {
                let mut cnt = 0;
                for v in pos.iter() {
                    let l = v.partition_point(|&x| x < q[0]);
                    let r = v.partition_point(|&x| x <= q[1]);
                    if (r - l) % 2 == 1 {
                        cnt += 1;
                    }
                }
                cnt / 2 <= q[2]
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn can_make_pali_queries() {
        assert_eq!(
            Solution::can_make_pali_queries(
                String::from("abcda"),
                vec_vec![[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]]
            ),
            vec![true, false, false, true, true]
        );
        assert_eq!(
            Solution::can_make_pali_queries(String::from("lyb"), vec_vec![[0, 1, 0], [2, 2, 1]]),
            vec![false, true]
        );
    }
}
