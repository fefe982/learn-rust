// https://leetcode.com/problems/count-connected-components-in-lcm-graph/
// 3378. Count Connected Components in a 2D Grid
pub struct Solution;
impl Solution {
    pub fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
        let k = threshold as usize;
        let mut set = (0..=k).collect::<Vec<_>>();
        let getp = |set: &mut Vec<usize>, n: usize| {
            let mut p = n;
            while p != set[p] {
                p = set[p];
            }
            set[n] = p;
            p
        };
        for &n in &nums {
            if n > threshold {
                continue;
            }
            let n = n as usize;
            if set[n] != n {
                continue;
            }
            let pn = n;
            for j in 2..=k / n {
                let pj = getp(&mut set, n * j);
                set[pj] = pn;
            }
        }
        nums.into_iter()
            .filter(|&n| {
                if n > threshold || set[n as usize] == n as usize {
                    true
                } else {
                    false
                }
            })
            .count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_components() {
        assert_eq!(Solution::count_components(vec![2, 4, 8, 3, 9], 5), 4);
        assert_eq!(Solution::count_components(vec![2, 4, 8, 3, 9, 12], 10), 2)
    }
}
