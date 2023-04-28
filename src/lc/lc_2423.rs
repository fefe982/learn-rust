// https://leetcode.com/problems/remove-letter-to-equalize-frequency/
// 2423. Remove Letter To Equalize Frequency
pub struct Solution;
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut set = std::collections::HashMap::<u8, i32>::new();
        for &c in word.as_bytes() {
            *set.entry(c).or_default() += 1;
        }
        if set.len() <= 1 || set.len() == word.len() {
            return true;
        }
        let len = word.len();
        let mut good = true;
        if len % set.len() == 1 {
            let len = (len / set.len()) as i32;
            for &v in set.values() {
                if v != len && v != len + 1 {
                    good = false;
                }
            }
            if good {
                return true;
            }
        }
        if (len - 1) % (set.len() - 1) == 0 {
            let len = ((len - 1) / (set.len() - 1)) as i32;
            for &v in set.values() {
                if v != 1 && v != len {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equal_frequency() {
        assert_eq!(Solution::equal_frequency(String::from("abbbccc")), true);
        assert_eq!(Solution::equal_frequency(String::from("abcc")), true);
        assert_eq!(Solution::equal_frequency(String::from("aazz")), false);
    }
}
