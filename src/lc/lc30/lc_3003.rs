// https://leetcode.com/problems/maximize-the-number-of-partitions-after-operations/
// 3003. Maximize the Number of Partitions After Operations
pub struct Solution;
impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        if k == 26 {
            return 1;
        }
        let s = s.as_bytes();
        let len = s.len();
        let mut cnt = vec![vec![0; 26]; len + 1];
        let mut left = 0;
        let mut right = 0;
        let mut distinct = 0;
        let mut end = vec![0; len];
        let mut seg = vec![0; len];
        while right < len {
            let iright = (s[right] - b'a') as usize;
            cnt[right + 1] = cnt[right].clone();
            cnt[right + 1][iright] += 1;
            right += 1;
            if cnt[right][iright] - cnt[left][iright] == 1 {
                distinct += 1;
            }
            while distinct == k + 1 {
                end[left] = right - 1;
                let ileft = (s[left] - b'a') as usize;
                left += 1;
                if cnt[right][ileft] - cnt[left][ileft] == 0 {
                    distinct -= 1;
                }
            }
        }
        if left == 0 && distinct < k {
            return 1;
        }
        for i in left..len {
            seg[i] = 1;
            end[i] = len;
        }
        for i in (0..left).rev() {
            seg[i] = seg[end[i]] + 1;
        }
        let mut prev_seg_cnt = 0;
        let mut next_seg_start = 0;
        let mut max_seg_cnt = 0;
        let cnt_distinct = |s: usize, e: usize, inc: usize, dec: usize| -> i32 {
            let mut distinct = 0;
            for i in 0..26 {
                if cnt[e][i] - cnt[s][i] + (inc == i) as i32 - (dec == i) as i32 > 0 {
                    distinct += 1;
                };
            }
            distinct
        };
        for i in 0..len {
            if end[next_seg_start] <= i {
                prev_seg_cnt += 1;
                next_seg_start = end[next_seg_start];
            }
            let ichar = (s[i] - b'a') as usize;
            for j in 0..26 {
                if ichar == j {
                    continue;
                }
                if cnt_distinct(next_seg_start, len, j, ichar) <= k {
                    max_seg_cnt = max_seg_cnt.max(prev_seg_cnt + 1);
                    continue;
                }
                let mut nseg = next_seg_start;
                let mut nadd = 0;
                if cnt_distinct(next_seg_start, i + 1, j, ichar) > k {
                    nadd = 1;
                    nseg = i;
                    if cnt_distinct(nseg, len, j, ichar) <= k {
                        max_seg_cnt = max_seg_cnt.max(prev_seg_cnt + 2);
                        continue;
                    }
                }
                let mut high = len;
                let mut low = i;
                while low + 1 < high {
                    let mid = (low + high) / 2;
                    if cnt_distinct(nseg, mid, j, ichar) <= k {
                        low = mid;
                    } else {
                        high = mid;
                    }
                }
                max_seg_cnt = max_seg_cnt.max(prev_seg_cnt + 1 + nadd + seg[low]);
            }
        }
        max_seg_cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_partitions_after_operations() {
        assert_eq!(Solution::max_partitions_after_operations("accca".to_string(), 2), 3);
        assert_eq!(Solution::max_partitions_after_operations("aabaab".to_string(), 3), 1);
        assert_eq!(Solution::max_partitions_after_operations("xxyz".to_string(), 1), 4);
    }
}
