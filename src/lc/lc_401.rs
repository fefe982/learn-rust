// https://leetcode.com/problems/binary-watch/
// 401. Binary Watch
pub struct Solution;
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        if turned_on == 0 {
            return vec!["0:00".to_string()];
        }
        if turned_on > 8 {
            return vec![];
        }
        let get_time = |h: i32, m: i32| format!("{}:{:02}", h, m);
        let mut result = vec![];
        let mut t = (1 << turned_on) - 1;
        while t / 64 < 12 {
            if t % 64 < 60 {
                result.push(get_time(t / 64, t % 64));
            }
            t = t + (1 << t.trailing_zeros());
            t += (1 << (turned_on - t.count_ones() as i32)) - 1;
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_binary_watch() {
        assert_eq!(
            Solution::read_binary_watch(1),
            vec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
        );
        assert_eq!(Solution::read_binary_watch(9), Vec::<String>::new());
    }
}
