// https://leetcode.com/problems/fruits-into-baskets-ii/
// 3477. Fruit Into Baskets II
pub struct Solution;
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut n = 1;
        while n < fruits.len() {
            n *= 2;
        }
        let mut f = vec![0; n * 2];
        fn set(f: &mut Vec<i32>, idx: usize, v: i32, i: usize, l: usize, r: usize) {
            if idx < l || idx >= r {
                return;
            }
            f[i] = f[i].max(v);
            if l + 1 != r {
                let m = (l + r) / 2;
                set(f, idx, v, i * 2 + 1, l, m);
                set(f, idx, v, i * 2 + 2, m, r);
            }
        }
        fn find(f: &mut Vec<i32>, v: i32, i: usize, l: usize, r: usize) -> bool {
            if f[i] >= v {
                if l + 1 == r {
                    f[i] = 0;
                } else {
                    let m = (l + r) / 2;
                    if !find(f, v, i * 2 + 1, l, m) {
                        find(f, v, i * 2 + 2, m, r);
                    }
                    f[i] = f[i * 2 + 1].max(f[i * 2 + 2]);
                }
                return true;
            } else {
                return false;
            }
        }
        for (i, v) in baskets.into_iter().enumerate() {
            set(&mut f, i, v, 0, 0, n);
        }
        let mut ans = 0;
        for v in fruits {
            if !find(&mut f, v, 0, 0, n) {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_of_unplaced_fruits() {
        assert_eq!(Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]), 1);
        assert_eq!(Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]), 0);
    }
}
