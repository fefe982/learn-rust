// https://leetcode.com/problems/make-number-of-distinct-characters-equal/
// 2531. Make Number of Distinct Characters Equal
pub struct Solution;
impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let mut map1 = vec![0; 26];
        let mut map2 = vec![0; 26];
        for c in word1.chars() {
            map1[(c as u8 - 'a' as u8) as usize] += 1;
        }
        for c in word2.chars() {
            map2[(c as u8 - 'a' as u8) as usize] += 1;
        }
        let c1 = map1.iter().filter(|&&x| x > 0).count() as i32;
        let c2 = map2.iter().filter(|&&x| x > 0).count() as i32;
        if (c1 - c2).abs() > 2 {
            return false;
        }
        fn cnt(map: &Vec<i32>, add: usize, remove: usize) -> i32 {
            if add == remove {
                return 0;
            }
            let mut c = 0;
            if map[add] == 0 {
                c += 1;
            }
            if map[remove] == 1 {
                c -= 1;
            }
            c
        }
        for i in 0..26 {
            for j in 0..26 {
                if map1[i] > 0 && map2[j] > 0 {
                    if c1 + cnt(&map1, j, i) == c2 + cnt(&map2, i, j) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_it_possible() {
        assert_eq!(Solution::is_it_possible("ab".to_string(), "abcc".to_string()), false);
        assert_eq!(Solution::is_it_possible("ac".to_string(), "b".to_string()), false);
        assert_eq!(Solution::is_it_possible("abcc".to_string(), "aab".to_string()), true);
        assert_eq!(Solution::is_it_possible("abcde".to_string(), "fghij".to_string()), true);
    }
}
