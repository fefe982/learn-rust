// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/
// 3306. Count of Substrings Containing All Vowels and K Consonants II
pub struct Solution;
impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.as_bytes();
        let mut nc = vec![0; word.len()];
        let mut nci = word.len();
        for (i, &c) in word.iter().enumerate().rev() {
            nc[i] = nci;
            if c != b'a' && c != b'e' && c != b'i' && c != b'o' && c != b'u' {
                nci = i;
            }
        }
        let mut i = 0;
        let mut j = 0;
        let mut cnt = vec![0; 6];
        let mut res = 0;
        while j < word.len() {
            while j < word.len()
                && (cnt[0] == 0 || cnt[1] == 0 || cnt[2] == 0 || cnt[3] == 0 || cnt[4] == 0 || cnt[5] < k)
            {
                match word[j] {
                    b'a' => cnt[0] += 1,
                    b'e' => cnt[1] += 1,
                    b'i' => cnt[2] += 1,
                    b'o' => cnt[3] += 1,
                    b'u' => cnt[4] += 1,
                    _ => cnt[5] += 1,
                }
                j += 1;
            }
            while cnt[0] > 0 && cnt[1] > 0 && cnt[2] > 0 && cnt[3] > 0 && cnt[4] > 0 && cnt[5] >= k {
                if cnt[5] == k {
                    res += (nc[j - 1] - j + 1) as i64;
                }
                match word[i] {
                    b'a' => cnt[0] -= 1,
                    b'e' => cnt[1] -= 1,
                    b'i' => cnt[2] -= 1,
                    b'o' => cnt[3] -= 1,
                    b'u' => cnt[4] -= 1,
                    _ => cnt[5] -= 1,
                }
                i += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_of_substrings() {
        assert_eq!(Solution::count_of_substrings("aeioqq".to_string(), 12), 0);
        assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);
        assert_eq!(Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1), 3);
    }
}
