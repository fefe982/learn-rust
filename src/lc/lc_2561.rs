// https://leetcode.com/problems/rearranging-fruits/
// 2561. Rearranging Fruits
pub struct Solution;
impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut map = std::collections::BTreeMap::new();
        let mut min = i32::MAX;
        for b in basket1 {
            map.entry(b).and_modify(|e| *e += 1).or_insert(1);
            min = min.min(b);
        }
        for b in basket2 {
            map.entry(b).and_modify(|e| *e -= 1).or_insert(-1);
            min = min.min(b);
        }
        let mut vpos = vec![];
        let mut vneg = vec![];
        let mut sum = 0;
        for (k, v) in map {
            if v % 2 == 1 {
                return -1;
            }
            if v > 0 {
                vpos.push((k, v / 2));
            } else if v < 0 {
                vneg.push((k, -v / 2));
            }
        }
        if vneg.len() == 0 {
            return 0;
        }
        let mut ipos = 0;
        let mut ineg = vneg.len() - 1;
        loop {
            let cnt = vpos[ipos].1.min(vneg[ineg].1);
            sum += cnt as i64 * vpos[ipos].0.min(vneg[ineg].0).min(min * 2) as i64;
            vpos[ipos].1 -= cnt;
            vneg[ineg].1 -= cnt;
            if vpos[ipos].1 == 0 {
                ipos += 1;
            }
            if vneg[ineg].1 == 0 {
                if ineg == 0 {
                    return sum;
                }
                ineg -= 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost() {
        assert_eq!(
            Solution::min_cost(
                vec![84, 80, 43, 8, 80, 88, 43, 14, 100, 88],
                vec![32, 32, 42, 68, 68, 100, 42, 84, 14, 8]
            ),
            48
        );
        assert_eq!(Solution::min_cost(vec![5, 8, 15, 7], vec![5, 7, 8, 15]), 0);
        assert_eq!(Solution::min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]), 1);
        assert_eq!(Solution::min_cost(vec![2, 3, 4, 1], vec![3, 2, 5, 1]), -1);
    }
}
