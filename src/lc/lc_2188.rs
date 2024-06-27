// https://leetcode.com/problems/minimum-time-to-finish-the-race/
// 2188. Minimum Time to Finish the Race
pub struct Solution;
impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        let mut min_time = vec![i64::MAX; num_laps as usize];
        let mut max_i = 0;
        for tire in tires {
            let mut lap = tire[0] as i64;
            let first_lap = lap;
            let mut sum = lap;
            let mul = tire[1] as i64;
            for i in 0..num_laps as usize {
                min_time[i] = min_time[i].min(sum);
                lap *= mul;
                if lap > change_time as i64 + first_lap {
                    max_i = max_i.max(i + 1);
                    break;
                }
                sum += lap;
            }
        }
        for i in 1..num_laps as usize {
            for j in 1..=max_i.min(i) {
                min_time[i] = min_time[i].min(min_time[i - j] + min_time[j - 1] + change_time as i64);
            }
        }
        min_time[num_laps as usize - 1] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_finish_time() {
        assert_eq!(
            Solution::minimum_finish_time(
                vec_vec![
                    [3, 4],
                    [84, 2],
                    [63, 8],
                    [72, 8],
                    [82, 7],
                    [83, 6],
                    [23, 2],
                    [77, 5],
                    [51, 10],
                    [28, 2],
                    [47, 9],
                    [8, 3],
                    [48, 3],
                    [56, 3],
                    [8, 10],
                    [66, 6],
                    [92, 9],
                    [44, 6],
                    [23, 5],
                    [5, 6],
                    [86, 9],
                    [13, 10],
                    [91, 3],
                    [2, 2],
                    [8, 4],
                    [67, 8],
                    [63, 6],
                    [52, 5],
                    [42, 10],
                    [3, 9],
                    [66, 5],
                    [35, 10],
                    [63, 6],
                    [65, 6],
                    [22, 8],
                    [40, 9],
                    [43, 4],
                    [73, 9],
                    [81, 5],
                    [32, 2],
                    [30, 5],
                    [80, 9],
                    [50, 4],
                    [35, 4],
                    [52, 7],
                    [11, 5],
                    [7, 8],
                    [68, 3],
                    [54, 8],
                    [49, 8]
                ],
                90,
                87
            ),
            2526
        );
        assert_eq!(Solution::minimum_finish_time(vec_vec![[2, 3], [3, 4]], 5, 4), 21);
        assert_eq!(
            Solution::minimum_finish_time(vec_vec![[1, 10], [2, 2], [3, 4]], 6, 5),
            25
        );
    }
}
