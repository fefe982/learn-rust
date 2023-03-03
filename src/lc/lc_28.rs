// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
// 28. Find the Index of the First Occurrence in a String
pub struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_bytes = needle.as_bytes();
        let haystack_bytes = haystack.as_bytes();
        if needle_bytes.len() > haystack_bytes.len() {
            return -1;
        }
        if needle_bytes.len() == haystack_bytes.len() {
            for (n, h) in needle_bytes.iter().zip(haystack_bytes.iter()) {
                if *n != *h {
                    return -1;
                }
            }
            return 0;
        }
        let mut next: Vec<usize> = Vec::with_capacity(needle_bytes.len());
        next.push(0);
        let mut i = 0;
        let mut j = 1;
        while j < needle_bytes.len() {
            if needle_bytes[i] == needle_bytes[j] {
                next.push(i + 1);
                i += 1;
                j += 1;
            } else if i != 0 {
                i = next[i - 1];
            } else {
                next.push(0);
                j += 1;
            }
        }
        i = 0;
        j = 0;
        while j + needle_bytes.len() - i <= haystack_bytes.len() {
            if needle_bytes[i] == haystack_bytes[j] {
                i += 1;
                j += 1;
                if i == needle_bytes.len() {
                    return (j - needle_bytes.len()) as i32;
                }
            } else if i == 0 {
                j += 1;
            } else {
                i = next[i - 1];
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn str_str() {
        assert_eq!(
            Solution::str_str(String::from("sadbutsad"), String::from("sad")),
            0
        );
        assert_eq!(
            Solution::str_str(String::from("leetcode"), String::from("leeto")),
            -1
        );
    }
}
