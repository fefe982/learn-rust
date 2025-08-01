// https://leetcode.com/problems/handling-sum-queries-after-update/
// 2569. Handling Sum Queries After Update
pub struct Solution;
impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut sum = nums2.iter().fold(0i64, |acc, &n| acc + n as i64);
        let mut res = vec![];
        let mut n1 = vec![0u64; nums1.len() / 64 + if nums1.len() % 64 != 0 { 1 } else { 0 }];
        for (i, &n) in nums1.iter().enumerate() {
            if n == 1 {
                n1[i / 64] |= 1 << (i % 64);
            }
        }
        for q in queries {
            match q.as_slice() {
                &[1, l, r] => {
                    let l = l as usize;
                    let r = r as usize;
                    let mut il = l / 64;
                    let mut ir = r / 64;
                    let pl = l % 64;
                    let pr = r % 64 + 1;
                    if il == ir {
                        if pr == 64 && pl == 0 {
                            ir += 1;
                        } else {
                            n1[il] ^= ((1 << (pr - pl)) - 1) << pl;
                        }
                    } else {
                        if pl > 0 {
                            n1[il] ^= ((1 << (64 - pl)) - 1) << pl;
                            il += 1;
                        }
                        if pr < 64 {
                            n1[ir] ^= (1 << pr) - 1;
                        } else {
                            ir += 1;
                        }
                    }
                    for i in il..ir {
                        n1[i] ^= !0;
                    }
                }
                &[2, p, _] => {
                    sum += n1.iter().fold(0i64, |acc, &n| acc + n.count_ones() as i64) * p as i64
                }
                [3, _, _] => res.push(sum),
                _ => unreachable!(),
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn handle_query() {
        assert_eq!(
            Solution::handle_query(
                vec![0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1],
                vec![
                    30, 46, 43, 34, 39, 16, 14, 41, 22, 11, 32, 2, 44, 12, 22, 36, 44, 49, 50, 10,
                    33, 7, 42
                ],
                vec_vec![
                    [1, 15, 21],
                    [3, 0, 0],
                    [3, 0, 0],
                    [2, 21, 0],
                    [2, 13, 0],
                    [3, 0, 0]
                ]
            ),
            vec![679, 679, 1053]
        );
        assert_eq!(
            Solution::handle_query(
                vec![1, 0, 1],
                vec![0, 0, 0],
                vec_vec![[1, 1, 1], [2, 1, 0], [3, 0, 0]]
            ),
            vec![3]
        );
        assert_eq!(
            Solution::handle_query(vec![1], vec![5], vec_vec![[2, 0, 0], [3, 0, 0]]),
            vec![5]
        );
    }
}
