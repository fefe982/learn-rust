// https://leetcode.com/problems/slowest-key/
// 1629. Slowest Key
pub struct Solution;
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut release_times = release_times;
        for i in (1..release_times.len()).rev() {
            release_times[i] -= release_times[i - 1];
        }
        let mut max_t = 0;
        let mut max_c = 'a';
        for (t, c) in release_times.into_iter().zip(keys_pressed.chars()) {
            if t > max_t || (t == max_t && c > max_c) {
                max_t = t;
                max_c = c;
            }
        }
        max_c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn slowest_key() {
        assert_eq!(Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_string()), 'c');
        assert_eq!(
            Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string()),
            'a'
        );
    }
}
