// https://leetcode.com/problems/total-waviness-of-numbers-in-range-ii/
// 3753. Total Waviness of Numbers in Range II
pub struct Solution;
impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        let nlow = num1
            .to_string()
            .as_bytes()
            .into_iter()
            .map(|x| (x - b'0') as usize)
            .rev()
            .collect::<Vec<usize>>();
        let nhigh = num2
            .to_string()
            .as_bytes()
            .into_iter()
            .map(|x| (x - b'0') as usize)
            .rev()
            .collect::<Vec<usize>>();
        fn dfs(
            i: usize,
            last_cmp: usize,
            last_digit: usize,
            limit_low: usize,
            limit_high: usize,
            nlow: &Vec<usize>,
            nhigh: &Vec<usize>,
            cache: &mut Vec<Vec<Vec<Vec<Vec<(i64, i64)>>>>>,
        ) -> (i64, i64) {
            if i == 0 {
                return (0, 1);
            }
            let i = i - 1;
            if cache[i][last_cmp][last_digit][limit_low][limit_high] != (-1, -1) {
                return cache[i][last_cmp][last_digit][limit_low][limit_high];
            }
            let mut waviness = 0;
            let mut cnt = 0;
            let last_is_num = limit_low == 0 || i + 1 < nlow.len();
            let bound_low = if limit_low == 1 && i < nlow.len() { nlow[i] } else { 0 };
            let bound_high = if limit_high == 1 { nhigh[i] } else { 9 };
            for d in bound_low..=bound_high {
                let this_cmp = if last_is_num {
                    match d.cmp(&last_digit) {
                        std::cmp::Ordering::Less => 0,
                        std::cmp::Ordering::Equal => 1,
                        std::cmp::Ordering::Greater => 2,
                    }
                } else {
                    1
                };
                let (subwaviness, subcnt) = dfs(
                    i,
                    this_cmp,
                    d,
                    if limit_low == 1 && d == bound_low { 1 } else { 0 },
                    if limit_high == 1 && d == bound_high { 1 } else { 0 },
                    nlow,
                    nhigh,
                    cache,
                );
                waviness += subwaviness;
                if (this_cmp as i32 - 1) * (last_cmp as i32 - 1) < 0 {
                    waviness += subcnt;
                }
                cnt += subcnt;
            }
            cache[i][last_cmp][last_digit as usize][limit_low][limit_high] = (waviness, cnt);
            (waviness, cnt)
        }
        dfs(
            nhigh.len(),
            1,
            0,
            1,
            1,
            &nlow,
            &nhigh,
            &mut vec![vec![vec![vec![vec![(-1, -1); 2]; 2]; 10]; 3]; nhigh.len()],
        )
        .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_waviness() {
        assert_eq!(Solution::total_waviness(120, 130), 3);
        assert_eq!(Solution::total_waviness(198, 202), 3);
        assert_eq!(Solution::total_waviness(4848, 4848), 2);
    }
}
