// https://leetcode.com/problems/minimum-possible-maximum-waiting-time/solutions/8435106/optimized-binary-search-by-rainboy1-os9g/
// 4009. Minimum Possible Maximum Waiting Time
pub struct Solution;
impl Solution {
    fn dfs(
        demand: &Vec<i32>,
        fuel: &mut [usize; 2],
        wait: [usize; 2],
        limit: usize,
        i: usize,
        // cache: &mut Vec<[[[[i32; 21]; 21]; 51]; 51]>,
        cache: &mut std::collections::HashMap<(usize, usize, usize, usize, usize), i32>,
    ) -> i32 {
        if i == demand.len() {
            return 0;
        }
        if cache.contains_key(&(i, fuel[0], fuel[1], wait[0], wait[1])) {
            return cache[&(i, fuel[0], fuel[1], wait[0], wait[1])];
        }
        let d = demand[i] as usize;
        let mut r = 0;
        for j in 0..2 {
            if fuel[j] < d || wait[j] > limit {
                continue;
            }
            fuel[j] -= d;
            let mut w = [0, 0];
            w[j] = d;
            w[1 - j] = wait[1 - j].saturating_sub(wait[j]);
            r = r.max(Self::dfs(demand, fuel, w, limit, i + 1, cache) + 1);
            fuel[j] += d;
        }
        cache.insert((i, fuel[0], fuel[1], wait[0], wait[1]), r);
        r
    }
    pub fn min_max_waiting_time(demand: Vec<i32>, fuel: Vec<i32>) -> i32 {
        let mut r = -1;
        let mut c = 0;
        for i in 0..=20 {
            let mut cache = std::collections::HashMap::new();
            let mut fuel = [fuel[0] as usize, fuel[1] as usize];
            let nc = Self::dfs(&demand, &mut fuel, [0, 0], i, 0, &mut cache);
            if nc > c {
                r = i as i32;
                c = nc;
            }
            if c == demand.len() as i32 {
                break;
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_max_waiting_time() {
        assert_eq!(
            Solution::min_max_waiting_time(vec![6, 9, 6, 2, 1, 8, 5, 9, 10, 4], vec![11, 14]),
            6
        );
        assert_eq!(
            Solution::min_max_waiting_time(vec![6, 7, 4, 7, 4, 4, 4, 6], vec![18, 22]),
            6
        );
        assert_eq!(Solution::min_max_waiting_time(vec![20, 9], vec![24, 45]), 0);
        assert_eq!(Solution::min_max_waiting_time(vec![6, 8, 4, 6, 5], vec![16, 13]), 6);
        assert_eq!(Solution::min_max_waiting_time(vec![10, 15], vec![12, 17]), 0);
        assert_eq!(Solution::min_max_waiting_time(vec![10, 5], vec![8, 8]), -1);
    }
}
