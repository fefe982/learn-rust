// https://leetcode.com/problems/number-of-balanced-integers-in-a-range/
// 3791. Number of Balanced Integers in a Range
pub struct Solution;
impl Solution {
    pub fn count_balanced(low: i64, high: i64) -> i64 {
        let mut nhigh = high
            .to_string()
            .as_bytes()
            .into_iter()
            .map(|x| (x - b'0') as usize)
            .rev()
            .collect::<Vec<usize>>();
        if nhigh.len() % 2 == 1 {
            nhigh.push(0);
        }
        let mut nlow = low
            .to_string()
            .as_bytes()
            .into_iter()
            .map(|x| (x - b'0') as usize)
            .rev()
            .collect::<Vec<usize>>();
        while nlow.len() < nhigh.len() {
            nlow.push(0);
        }
        fn dfs(
            i: usize,
            sum: i32,
            shift: i32,
            limit_low: usize,
            limit_high: usize,
            nlow: &Vec<usize>,
            nhigh: &Vec<usize>,
            cache: &mut Vec<Vec<Vec<Vec<i64>>>>,
        ) -> i64 {
            if i == 0 {
                if sum == 0 {
                    return 1;
                } else {
                    return 0;
                }
            }
            let left = i as i32;
            let even = left / 2;
            let odd = left - even;
            if sum - odd * 9 > 0 || sum + even * 9 < 0 {
                return 0;
            }
            let i = i - 1;
            if cache[i][(sum + shift) as usize][limit_low][limit_high] != -1 {
                return cache[i][(sum + shift) as usize][limit_low][limit_high];
            }
            let mut cnt = 0;
            let bound_low = if limit_low == 1 && i < nlow.len() { nlow[i] } else { 0 };
            let bound_high = if limit_high == 1 { nhigh[i] } else { 9 };
            let mul = if i % 2 == 0 { -1 } else { 1 };
            for d in bound_low..=bound_high {
                cnt += dfs(
                    i,
                    sum + mul * (d as i32),
                    shift,
                    if limit_low == 1 && d == bound_low { 1 } else { 0 },
                    if limit_high == 1 && d == bound_high { 1 } else { 0 },
                    nlow,
                    nhigh,
                    cache,
                );
            }
            cache[i][(sum + shift) as usize][limit_low][limit_high] = cnt;
            cnt
        }
        dfs(
            nhigh.len(),
            0,
            9 * (nhigh.len() as i32 / 2),
            1,
            1,
            &nlow,
            &nhigh,
            &mut vec![vec![vec![vec![-1; 2]; 2]; 9 as usize * nhigh.len() + 1]; nhigh.len()],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_balanced() {
        assert_eq!(Solution::count_balanced(1, 100), 9);
        assert_eq!(Solution::count_balanced(120, 129), 1);
        assert_eq!(Solution::count_balanced(1234, 1234), 0);
    }
}
