// https://leetcode.com/problems/3sum-with-multiplicity/
// 923. 3Sum With Multiplicity
pub struct Solution;
impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        const MOD: i64 = 1_000_000_007i64;
        let mut cnt = vec![0; 101];
        for n in arr {
            cnt[n as usize] += 1;
        }
        let mut total = 0;
        let target = target as usize;
        for i in 0..=target / 3 {
            if cnt[i] == 0 {
                continue;
            }
            let ci = cnt[i] as i64;
            for j in i..=100.min((target - i) / 2) {
                if (i == j && cnt[i] < 2) || cnt[j] == 0 {
                    continue;
                }
                let cj = cnt[j] as i64;
                let k = target - i - j;
                if k > 100 {
                    continue;
                }
                if (j == k && cnt[j] < 2) || cnt[k] == 0 {
                    continue;
                }
                let ck = cnt[k] as i64;
                if i == j {
                    if j == k {
                        if ci >= 3 {
                            total = (total + ci * (ci - 1) * (ci - 2) / 6) % MOD;
                        }
                    } else {
                        total = (total + ci * (ci - 1) * ck / 2) % MOD;
                    }
                } else {
                    if j == k {
                        total = (total + ci * cj * (cj - 1) / 2) % MOD;
                    } else {
                        total = (total + ci * cj * ck) % MOD;
                    }
                }
            }
        }
        total as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn three_sum_multi() {
        assert_eq!(
            Solution::three_sum_multi(
                vec![
                    92, 4, 59, 23, 100, 16, 7, 15, 3, 78, 98, 17, 77, 33, 83, 15, 87, 35, 54, 72, 58, 14, 87, 47, 58,
                    31, 72, 58, 87, 22, 25, 54, 27, 53, 13, 54, 61, 12, 96, 24, 35, 43, 94, 1, 88, 76, 89, 89, 41, 56,
                    61, 65, 60, 91, 89, 79, 86, 52, 27, 2, 97, 46, 50, 46, 87, 93, 71, 87, 95, 78, 65, 10, 35, 51, 34,
                    66, 61, 7, 49, 38, 10, 1, 88, 37, 50, 84, 35, 20, 7, 83, 51, 85, 11, 12, 89, 93, 54, 23, 36, 95,
                    100, 19, 82, 67, 96, 77, 53, 56, 51, 16, 54, 7, 30, 68, 78, 13, 38, 52, 91, 44, 54, 17, 21, 44, 4,
                    10, 85, 19, 11, 88, 73, 36, 47, 53, 3, 21, 41, 24, 60, 53, 88, 35, 48, 89, 35, 3, 43, 85, 45, 67,
                    56, 78, 44, 49, 29, 35, 68, 96, 29, 21, 51, 17, 52, 99, 3, 48, 65, 51, 22, 38, 77, 81, 30, 64, 99,
                    35, 88, 72, 73, 29, 29, 2
                ],
                105
            ),
            5166
        );
        assert_eq!(Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8), 20);
        assert_eq!(Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5), 12);
        assert_eq!(Solution::three_sum_multi(vec![2, 1, 3], 6), 1);
    }
}
