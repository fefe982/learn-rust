// https://leetcode.com/problems/shortest-matching-substring/
// 3455. Shortest Matching Substring
pub struct Solution;
impl Solution {
    fn get_pos(s: &str, p: &str) -> Vec<usize> {
        if p.len() == 0 {
            return (0..=s.len()).collect::<Vec<usize>>();
        }
        let p = p.as_bytes();
        let mut pv = vec![0; p.len()];
        let mut next = 0;
        for i in 1..p.len() {
            while next > 0 && p[next] != p[i] {
                next = pv[next - 1];
            }
            if p[next] == p[i] {
                next += 1;
            }
            pv[i] = next;
        }
        let s = s.as_bytes();
        let mut len = 0;
        let mut ans = vec![];
        for i in 0..s.len() {
            while len > 0 && p[len] != s[i] {
                len = pv[len - 1];
            }
            if p[len] == s[i] {
                len += 1;
            }
            if len == p.len() {
                ans.push(i + 1 - len);
                len = pv[len - 1];
            }
        }
        ans
    }
    pub fn shortest_matching_substring(s: String, p: String) -> i32 {
        let psplit = p.split('*').collect::<Vec<&str>>();
        let pos0 = Self::get_pos(&s, psplit[0]);
        let l0 = psplit[0].len();
        let pos1 = Self::get_pos(&s, psplit[1]);
        let l1 = psplit[1].len();
        let pos2 = Self::get_pos(&s, psplit[2]);
        let l2 = psplit[2].len();
        if pos0.len() == 0 || pos1.len() == 0 || pos2.len() == 0 {
            return -1;
        }
        let mut ans = usize::MAX;
        let mut i0 = 0;
        let mut i1 = 0;
        let mut i2 = 0;
        while pos1[i1] < pos0[i0] + l0 && i1 + 1 < pos1.len() {
            i1 += 1;
        }
        while i1 < pos1.len() {
            while i0 + 1 < pos0.len() && pos0[i0 + 1] + l0 <= pos1[i1] {
                i0 += 1;
            }
            while i2 < pos2.len() && pos1[i1] + l1 > pos2[i2] {
                i2 += 1;
            }
            if i2 >= pos2.len() {
                break;
            }
            ans = ans.min(pos2[i2] - pos0[i0] + l2);
            i1 += 1;
        }
        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortest_matching_substring() {
        assert_eq!(
            Solution::shortest_matching_substring("abaacbaecebce".to_string(), "ba*c*ce".to_string()),
            8
        );
        assert_eq!(
            Solution::shortest_matching_substring("baccbaadbc".to_string(), "cc*baa*adb".to_string()),
            -1
        );
        assert_eq!(
            Solution::shortest_matching_substring("a".to_string(), "**".to_string()),
            0
        );
        assert_eq!(
            Solution::shortest_matching_substring("madlogic".to_string(), "*adlogi*".to_string()),
            6
        );
    }
}
