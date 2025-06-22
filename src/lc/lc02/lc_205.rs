// https://leetcode.com/problems/isomorphic-strings/
// 205. Isomorphic Strings
pub struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = vec![0; 128];
        let mut rmap = vec![0; 128];
        for (c1, c2) in s.bytes().zip(t.bytes()) {
            if map[c1 as usize] != 0 && map[c1 as usize] != c2 {
                return false;
            }
            if rmap[c2 as usize] != 0 && rmap[c2 as usize] != c1 {
                return false;
            }
            map[c1 as usize] = c2;
            rmap[c2 as usize] = c1;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic(String::from("badc"), String::from("baba")),
            false
        );
        assert_eq!(Solution::is_isomorphic(String::from("egg"), String::from("add")), true);
        assert_eq!(Solution::is_isomorphic(String::from("foo"), String::from("bar")), false);
        assert_eq!(
            Solution::is_isomorphic(String::from("paper"), String::from("title")),
            true
        );
    }
}
