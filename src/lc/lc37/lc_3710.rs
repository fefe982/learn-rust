// https://leetcode.com/problems/maximum-partition-factor/
// 3710. Maximum Partition Factor
pub struct Solution;
impl Solution {
    pub fn max_partition_factor(points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();
        if len <= 2 {
            return 0;
        }
        let mut dis = Vec::with_capacity(len * (len - 1) / 2);
        for i in 0..len {
            for j in i + 1..len {
                let dx = (points[i][0] - points[j][0]).abs();
                let dy = (points[i][1] - points[j][1]).abs();
                dis.push((dx + dy, i, j));
            }
        }
        let mut g = (0..len).map(|x| (x, 0)).collect::<Vec<(usize, i32)>>();
        dis.sort();
        fn get(g: &mut Vec<(usize, i32)>, i: usize) -> (usize, i32) {
            if g[i].0 != i {
                let (ih, id) = get(g, g[i].0);
                g[i].0 = ih;
                g[i].1 ^= id;
            }
            g[i]
        }
        for (d, i, j) in dis {
            let (ih, id) = get(&mut g, i);
            let (jh, jd) = get(&mut g, j);
            if ih != jh {
                g[ih].0 = jh;
                g[ih].1 = 1 ^ id ^ jd;
            } else {
                if id == jd {
                    return d;
                }
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_partition_factor() {
        assert_eq!(
            Solution::max_partition_factor(vec_vec![[10792, -62802], [60833, -50454], [52675, -33454]]),
            71231
        );
        assert_eq!(
            Solution::max_partition_factor(vec_vec![[0, 0], [0, 2], [2, 0], [2, 2]]),
            4
        );
        assert_eq!(Solution::max_partition_factor(vec_vec![[0, 0], [0, 1], [10, 0]]), 11);
    }
}
