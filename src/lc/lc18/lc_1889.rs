// https://leetcode.com/problems/minimum-space-wasted-from-packaging/
// 1889. Minimum Space Wasted From Packaging
pub struct Solution;
impl Solution {
    pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        let mut packages = packages;
        let boxes = boxes;
        let mut ans = i64::MAX;
        packages.sort_unstable();
        let sum = packages.iter().fold(0i64, |acc, &x| acc + (x as i64));
        let &max_package = packages.last().unwrap();
        for mut b in boxes {
            b.sort_unstable();
            if *b.last().unwrap() < max_package {
                continue;
            }
            let mut last = 0;
            let mut box_sum = 0i64;
            for x in b {
                let cur = packages.partition_point(|&y| y <= x);
                box_sum += x as i64 * (cur - last) as i64;
                last = cur;
            }
            ans = ans.min(box_sum - sum);
        }
        if ans == i64::MAX {
            -1
        } else {
            (ans % 1_0000_0000_7) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_wasted_space() {
        assert_eq!(Solution::min_wasted_space(vec![2, 3, 5], vec_vec![[4, 8], [2, 8]]), 6);
        assert_eq!(
            Solution::min_wasted_space(vec![2, 3, 5], vec_vec![[1, 4], [2, 3], [3, 4]]),
            -1
        );
        assert_eq!(
            Solution::min_wasted_space(vec![3, 5, 8, 10, 11, 12], vec_vec![[12], [11, 9], [10, 5, 14]]),
            9
        );
    }
}
