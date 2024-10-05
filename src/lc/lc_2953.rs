// https://leetcode.com/problems/count-complete-substrings/
// 2953. Count Complete Substrings
pub struct Solution;
impl Solution {
    pub fn count_complete_substrings(word: String, k: i32) -> i32 {
        let mut cnt = vec![vec![0; 26]; 26];
        let mut compelete = vec![0; 26];
        let word = word.as_bytes();
        let mut left = 0;
        let mut res = 0;
        let k = k as usize;
        for i in 0..word.len() {
            if i > 0 && (word[i] as i32 - word[i - 1] as i32).abs() > 2 {
                cnt.iter_mut().for_each(|x| x.fill(0));
                compelete.fill(0);
                left = i;
            }
            let idx = (word[i] - b'a') as usize;
            let len = i - left + 1;
            let mut seg = len / k + 1;
            if seg > 1 && seg <= 26 && len % k == 0 {
                cnt[seg - 1] = cnt[seg - 2].clone();
                compelete[seg - 1] = compelete[seg - 2];
            }
            seg = seg.min(26);
            for j in 0..seg {
                cnt[j][idx] += 1;
                if cnt[j][idx] == k {
                    compelete[j] += 1;
                }
                if len > k * (j + 1) {
                    let idxseg = (word[i - k * (j + 1)] - b'a') as usize;
                    cnt[j][idxseg] -= 1;
                    if cnt[j][idxseg] == k - 1 {
                        compelete[j] -= 1;
                    }
                }
                if compelete[j] == j + 1 {
                    res += 1;
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_complete_substrings() {
        assert_eq!(
            Solution::count_complete_substrings("aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz".to_string(), 2),
            351
        );
        assert_eq!(Solution::count_complete_substrings("igigee".to_string(), 2), 3);
        assert_eq!(Solution::count_complete_substrings("igigee".to_string(), 2), 3);
        assert_eq!(Solution::count_complete_substrings("aaabbbccc".to_string(), 3), 6);
    }
}
