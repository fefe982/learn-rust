// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii/
// 3440. Reschedule Meetings for Maximum Free Time II
pub struct Solution;
impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut gaps = vec![0; n + 1];
        let mut top_gap = vec![0; 3];
        fn insert_gap(top_gap: &mut Vec<i32>, gap: i32) {
            if gap <= top_gap[2] {
                return;
            } else {
                top_gap[2] = top_gap[1];
            }
            if gap <= top_gap[1] {
                top_gap[2] = gap;
                return;
            } else {
                top_gap[1] = top_gap[0];
            }
            if gap <= top_gap[0] {
                top_gap[1] = gap;
            } else {
                top_gap[0] = gap;
            }
        }
        gaps[0] = start_time[0];
        insert_gap(&mut top_gap, start_time[0]);
        for i in 1..n {
            gaps[i] = start_time[i] - end_time[i - 1];
            insert_gap(&mut top_gap, gaps[i]);
        }
        gaps[n] = event_time - end_time[n - 1];
        insert_gap(&mut top_gap, gaps[n]);
        let mut max = 0;
        for i in 0..n {
            let mut gap = gaps[i] + gaps[i + 1];
            let d = end_time[i] - start_time[i];
            let lgap = gaps[i].max(gaps[i + 1]);
            let sgap = gap - lgap;
            if lgap < top_gap[0] {
                if d <= top_gap[0] {
                    gap += d;
                }
            } else if sgap < top_gap[1] {
                if d <= top_gap[1] {
                    gap += d;
                }
            } else if d <= top_gap[2] {
                gap += d;
            }
            max = max.max(gap);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_free_time() {
        assert_eq!(Solution::max_free_time(5, vec![1, 3], vec![2, 5]), 2);
        assert_eq!(Solution::max_free_time(10, vec![0, 7, 9], vec![1, 8, 10]), 7);
        assert_eq!(Solution::max_free_time(10, vec![0, 3, 7, 9], vec![1, 3, 8, 10]), 6);
    }
}
