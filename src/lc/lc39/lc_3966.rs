// https://leetcode.com/problems/count-good-integers-in-a-range/
// 3966. Count Good Integers in a Range
pub struct Solution;
impl Solution {
    pub fn good_integers(l: i64, r: i64, k: i32) -> i64 {
        let nhigh = r
            .to_string()
            .as_bytes()
            .into_iter()
            .map(|x| (x - b'0') as usize)
            .rev()
            .collect::<Vec<usize>>();
        let mut nlow = l
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
            k: usize,
            last: usize,
            limit_low: usize,
            limit_high: usize,
            nlow: &Vec<usize>,
            nhigh: &Vec<usize>,
            cache: &mut Vec<Vec<Vec<Vec<i64>>>>,
        ) -> i64 {
            if i == 0 {
                return 1;
            }
            let i = i - 1;
            if cache[i][last][limit_low][limit_high] != -1 {
                return cache[i][last][limit_low][limit_high];
            }
            let mut cnt = 0;
            let have_unbound = last == 10 && nlow[i] == 0;
            let bound_low = if last == 10 {
                nlow[i].max(1)
            } else if limit_low == 1 {
                nlow[i]
            } else {
                0
            };
            let bound_high = if limit_high == 1 { nhigh[i] } else { 9 };
            for d in bound_low..=bound_high {
                if last != 10 && (last > d + k || d > last + k) {
                    continue;
                }
                cnt += dfs(
                    i,
                    k,
                    d,
                    if limit_low == 1 && d == nlow[i] { 1 } else { 0 },
                    if limit_high == 1 && d == bound_high { 1 } else { 0 },
                    nlow,
                    nhigh,
                    cache,
                );
            }
            if have_unbound {
                cnt += dfs(i, k, 10, 1, 0, nlow, nhigh, cache);
            }
            cache[i][last][limit_low][limit_high] = cnt;
            cnt
        }
        dfs(
            nhigh.len(),
            k as usize,
            10,
            1,
            1,
            &nlow,
            &nhigh,
            &mut vec![vec![vec![vec![-1; 2]; 2]; 11]; nhigh.len()],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn good_integers() {
        assert_eq!(Solution::good_integers(15, 147, 7), 126);
        assert_eq!(Solution::good_integers(10, 15, 1), 3);
        assert_eq!(Solution::good_integers(201, 204, 2), 2);
    }
}
