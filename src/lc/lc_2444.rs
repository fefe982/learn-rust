// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/
// 2444. Count Subarrays With Fixed Bounds
pub struct Solution;
enum Seek {
    Min,
    Max,
    MinMax,
}
impl Solution {
    fn count_subarrays_eq(nums: &Vec<i32>, k: i32) -> i64 {
        let mut total: i64 = 0;
        let mut cnt = 0;
        for n in nums.iter() {
            if *n == k {
                cnt += 1;
            } else {
                total += (cnt + 1) * cnt / 2;
                cnt = 0
            }
        }
        total + (cnt + 1) * cnt / 2
    }

    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        if min_k == max_k {
            return Self::count_subarrays_eq(&nums, min_k);
        }
        let mut total: i64 = 0;
        let mut subtotal_invalid: i64 = 0;
        let mut last_bound: i64 = -1;
        let mut last_peek = -1;
        let mut seek = Seek::MinMax;
        let mut cnt_in_range = 0;
        for (idx, n) in nums.iter().enumerate() {
            if *n < min_k || *n > max_k {
                if last_bound != -1 {
                    let cnt = idx as i64 - last_bound;
                    subtotal_invalid += (cnt + 1) * cnt / 2;
                    total += (cnt_in_range + 1) * (cnt_in_range) / 2 - subtotal_invalid;
                }
                cnt_in_range = 0;
                subtotal_invalid = 0;
                last_bound = -1;
            } else {
                cnt_in_range += 1;
                if last_bound == -1 {
                    last_bound = idx as i64;
                    seek = Seek::MinMax;
                }
                if *n == min_k {
                    match seek {
                        Seek::MinMax => {
                            seek = Seek::Max;
                            last_peek = idx as i64;
                        }
                        Seek::Min => {
                            let mut cnt = idx as i64 - last_bound;
                            subtotal_invalid += (cnt + 1) * cnt / 2;
                            cnt = idx as i64 - last_peek - 1;
                            subtotal_invalid -= (cnt + 1) * cnt / 2;
                            last_bound = last_peek + 1;
                            seek = Seek::Max;
                            last_peek = idx as i64;
                        }
                        Seek::Max => {
                            last_peek = idx as i64;
                        }
                    }
                }
                if *n == max_k {
                    match seek {
                        Seek::MinMax => {
                            seek = Seek::Min;
                            last_peek = idx as i64;
                        }
                        Seek::Max => {
                            let mut cnt = idx as i64 - last_bound;
                            subtotal_invalid += (cnt + 1) * cnt / 2;
                            cnt = idx as i64 - last_peek - 1;
                            subtotal_invalid -= (cnt + 1) * cnt / 2;
                            last_bound = last_peek + 1;
                            seek = Seek::Min;
                            last_peek = idx as i64;
                        }
                        Seek::Min => {
                            last_peek = idx as i64;
                        }
                    }
                }
            }
        }
        if last_bound != -1 {
            let cnt = nums.len() as i64 - last_bound;
            subtotal_invalid += (cnt + 1) * cnt / 2;
            total += (cnt_in_range + 1) * (cnt_in_range) / 2 - subtotal_invalid;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    }
}
