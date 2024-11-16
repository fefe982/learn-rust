// https://leetcode.com/problems/alternating-groups-ii/
// 3208. Alternating Groups II
pub struct Solution;
impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut start = 0;
        let len = colors.len();
        if colors[len - 1] != colors[0] {
            start = len - 1;
            while start > 0 && colors[start - 1] != colors[start] {
                start -= 1;
            }
            if start == 0 {
                return len as i32;
            }
        }
        let mut s = start;
        let mut e = 0;
        let mut res = 0;
        loop {
            while e + 1 < len && colors[e + 1] != colors[e] {
                e += 1;
            }
            let l = ((e + len - s) % len + 1) as i32;
            if l >= k {
                res += l - k + 1;
            }
            s = e + 1;
            if s >= len || s == start {
                break;
            }
            e = s;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_alternating_groups() {
        assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3), 3);
        assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6), 2);
        assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4), 0);
    }
}
