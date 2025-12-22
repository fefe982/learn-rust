// https://leetcode.cn/problems/zui-xiao-tiao-yue-ci-shu/
// LCP 09. 最小跳跃次数
pub struct Solution;
impl Solution {
    pub fn min_jump(jump: Vec<i32>) -> i32 {
        let nj = jump.len();
        let mut nt = 1;
        while nt < nj {
            nt *= 2;
        }
        let mut tree = vec![i32::MAX; nt * 2];
        fn update(tree: &mut Vec<i32>, idx: usize, l: i32, r: i32, sl: i32, sr: i32, val: i32) {
            if sl >= r || sr <= l {
                return;
            }
            if sl <= l && sr >= r {
                tree[idx] = tree[idx].min(val);
                return;
            }
            let mid = (l + r) >> 1;
            update(tree, idx * 2 + 1, l, mid, sl, sr, val);
            update(tree, idx * 2 + 2, mid, r, sl, sr, val);
        }
        fn query(tree: &Vec<i32>, idx: usize, l: i32, r: i32, s: i32) -> i32 {
            if l + 1 == r {
                return tree[idx];
            }
            let mid = (l + r) >> 1;
            if s < mid {
                query(tree, idx * 2 + 1, l, mid, s)
            } else {
                query(tree, idx * 2 + 2, mid, r, s)
            }
            .min(tree[idx])
        }
        let nt = nt as i32;
        let nj = nj as i32;
        update(&mut tree, 0, 0, nt, 0, 1, 0);
        let mut res = nj;
        for i in 0..nj {
            let ji = jump[i as usize];
            let c = query(&tree, 0, 0, nt, i);
            if c > res {
                continue;
            }
            if i + ji >= nj as i32 {
                res = res.min(c + 1);
            } else {
                update(&mut tree, 0, 0, nt, i + ji, i + ji + 1, c + 1);
                if ji >= 2 {
                    update(&mut tree, 0, 0, nt, i + 1, i + ji, c + 2)
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_jump() {
        assert_eq!(
            Solution::min_jump(vec![3, 7, 6, 1, 4, 3, 7, 8, 1, 2, 8, 5, 9, 8, 3, 2, 7, 5, 1, 1]),
            6
        );
        assert_eq!(Solution::min_jump(vec![2, 5, 1, 1, 1, 1]), 3);
    }
}
