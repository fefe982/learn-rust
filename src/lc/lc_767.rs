// https://leetcode.com/problems/reorganize-string/
// 767. Reorganize String
pub struct Solution;
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let s = s.as_bytes();
        let mut m = std::collections::HashMap::<u8, i32>::new();
        for &c in s {
            *m.entry(c).or_default() += 1;
        }
        let mut q = std::collections::BinaryHeap::new();
        for (c, cnt) in m {
            q.push((cnt, std::cmp::Reverse(0), c));
        }
        let mut v = vec![];
        let mut last = 0;
        while let Some((cnt, _, c)) = q.pop() {
            if last != c {
                v.push(c);
                if cnt > 1 {
                    q.push((cnt - 1, std::cmp::Reverse(v.len()), c))
                }
                last = c;
            } else {
                if let Some((cnt2, _, c2)) = q.pop() {
                    v.push(c2);
                    if cnt2 > 1 {
                        q.push((cnt2 - 1, std::cmp::Reverse(v.len()), c2));
                    }
                    v.push(c);
                    if cnt > 1 {
                        q.push((cnt - 1, std::cmp::Reverse(v.len()), c));
                    }
                } else {
                    return "".to_string();
                }
            }
        }
        String::from_utf8(v).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reorganize_string() {
        assert_eq!(
            Solution::reorganize_string(String::from("aab")),
            String::from("aba")
        );
        assert_eq!(
            Solution::reorganize_string(String::from("aaab")),
            String::from("")
        )
    }
}
