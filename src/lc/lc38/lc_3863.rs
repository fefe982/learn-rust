// https://leetcode.com/problems/minimum-operations-to-sort-a-string/
// 3863. Minimum Operations to Sort a String
pub struct Solution;
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        let mut min = 26;
        let mut max = 0;
        let s = s.as_bytes();
        let len = s.len();
        let mut last = 0;
        let mut inc = true;
        for &c in s {
            let i = (c - b'a') as usize;
            if i < last {
                inc = false;
            }
            last = i;
            cnt[i] += 1;
            min = min.min(i);
            max = max.max(i);
        }
        let first = (s[0] - b'a') as usize;
        let last = (s[len - 1] - b'a') as usize;
        if len == 1 || inc {
            return 0;
        }
        if len == 2 {
            return -1;
        }
        if first == min || last == max {
            return 1;
        }
        if first == max && last == min && cnt[min] == 1 && cnt[max] == 1 {
            return 3;
        }
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations("dog".to_string()), 1);
        assert_eq!(Solution::min_operations("card".to_string()), 2);
        assert_eq!(Solution::min_operations("qf".to_string()), -1);
    }
}
