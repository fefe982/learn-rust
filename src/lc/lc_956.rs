// https://leetcode.com/problems/tallest-billboard/description/
// 956. Tallest Billboard
pub struct Solution;
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let mut diff = std::collections::HashMap::<i32, i32>::new();
        diff.insert(0, 0);
        for r in rods {
            let cur = diff.clone();
            for (d, l) in cur {
                diff.entry(d + r)
                    .and_modify(|e| *e = l.max(*e))
                    .or_insert(l);
                diff.entry((d - r).abs())
                    .and_modify(|e| *e = (l + d.min(r)).max(*e))
                    .or_insert(l + d.min(r));
            }
        }
        *diff.get(&0).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tallest_billboard() {
        assert_eq!(Solution::tallest_billboard(vec![1, 2, 3, 6]), 6);
        assert_eq!(Solution::tallest_billboard(vec![1, 2, 3, 4, 5, 6]), 10);
        assert_eq!(Solution::tallest_billboard(vec![1, 2]), 0);
    }
}
