// https://leetcode.cn/problems/ryfUiz/
// LCP 81. 与非的谜题
pub struct Solution;
impl Solution {
    pub fn get_nand_result(k: i32, arr: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        let k = k as usize;
        let mut s = vec![std::collections::BTreeSet::<usize>::new(); k];
        let mut arr = arr;
        let add = |s: &mut Vec<std::collections::BTreeSet<usize>>, n: i32, i: usize| {
            for j in 0..k {
                if n & (1 << j) == 0 {
                    s[j].insert(i);
                }
            }
        };
        let rep = |s: &mut Vec<std::collections::BTreeSet<usize>>, m: i32, n: i32, i: usize| {
            for j in 0..k {
                let mask = 1 << j;
                let mi = m & mask;
                let ni = n & mask;
                if mi != ni {
                    if ni != 0 {
                        s[j].remove(&i);
                    } else {
                        s[j].insert(i);
                    }
                }
            }
        };
        for (i, &n) in arr.iter().enumerate() {
            add(&mut s, n, i);
        }
        let mut ans = 0;
        let len = arr.len();
        for op in operations {
            if op[0] == 0 {
                let i = op[1] as usize;
                rep(&mut s, arr[i], op[2], i);
                arr[i] = op[2];
            } else {
                for i in 0..k {
                    let mask = 1 << i;
                    if let Some(&last) = s[i].iter().rev().next() {
                        if (len - last) % 2 == 1 {
                            ans ^= mask;
                        }
                    } else {
                        if len as i32 * op[1] % 2 == 0 {
                            ans ^= op[2] & mask;
                        } else {
                            ans ^= (op[2] ^ mask) & mask;
                        }
                    }
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::get_nand_result(3, vec![1, 2], vec_vec![[1, 2, 3], [0, 0, 3], [1, 2, 2]]),
            2
        );
        assert_eq!(
            Solution::get_nand_result(
                4,
                vec![4, 6, 4, 7, 10, 9, 11],
                vec_vec![[1, 5, 7], [1, 7, 14], [0, 6, 7], [1, 6, 5]]
            ),
            9
        );
    }
}
