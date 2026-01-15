// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/
// 2975. Maximum Square Area by Removing Fences from a Field
pub struct Solution;
impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        if m == n {
            return (((m - 1) as i64 * (n - 1) as i64) % MOD) as i32;
        }
        let mut hset = std::collections::HashSet::new();
        hset.insert(m - 1);
        for i in 0..h_fences.len() {
            let h = h_fences[i];
            hset.insert(h - 1);
            hset.insert(m - h);
            for j in 0..i {
                hset.insert((h - h_fences[j]).abs());
            }
        }
        let mut max = -1;
        if hset.contains(&(n - 1)) {
            max = n - 1;
        } else {
            for i in 0..v_fences.len() {
                let v = v_fences[i];
                if hset.contains(&(v - 1)) {
                    max = max.max(v - 1);
                }
                if hset.contains(&(n - v)) {
                    max = max.max(n - v);
                }
                for j in 0..i {
                    let d = (v - v_fences[j]).abs();
                    if hset.contains(&d) {
                        max = max.max(d);
                    }
                }
            }
        }
        if max == -1 {
            -1
        } else {
            ((max as i64 * max as i64) % MOD) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximize_square_area() {
        assert_eq!(Solution::maximize_square_area(3, 9, vec![2], vec![8, 6, 5, 4]), 4);
        assert_eq!(Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
        assert_eq!(Solution::maximize_square_area(6, 7, vec![2], vec![4]), -1);
    }
}
