// https://leetcode.com/problems/smallest-substring-with-identical-characters-ii/
// 3399. Smallest Substring With Identical Characters II
pub struct Solution;
impl Solution {
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len() as i32;
        let mut seg = vec![];
        let mut swap1 = 0;
        let mut max_seg = 0;
        let mut last = 0;
        let mut last_len = 0;
        for i in 0..n {
            let c = s[i as usize] as i32;
            if i % 2 == c % 2 {
                swap1 += 1;
            }
            if c != last {
                if last_len > 0 {
                    seg.push(last_len);
                    max_seg = max_seg.max(last_len);
                }
                last = c;
                last_len = 1;
            } else {
                last_len += 1;
            }
        }
        seg.push(last_len);
        max_seg = max_seg.max(last_len);
        if num_ops == 0 {
            return max_seg;
        }
        swap1 = swap1.min(n - swap1);
        if swap1 <= num_ops {
            return 1;
        }
        let mut min = 1;
        let mut max = max_seg;
        while min + 1 < max {
            let mid = (min + max) / 2;
            let sum = seg.iter().fold(0, |v, &s| v + s / (mid + 1));
            if sum <= num_ops {
                max = mid;
            } else {
                min = mid;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_length() {
        assert_eq!(Solution::min_length("000001".to_string(), 1), 2);
        assert_eq!(Solution::min_length("0000".to_string(), 2), 1);
        assert_eq!(Solution::min_length("0101".to_string(), 0), 1);
    }
}
