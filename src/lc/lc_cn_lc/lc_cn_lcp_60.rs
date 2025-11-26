// https://leetcode.cn/problems/WInSav/
// LCP 60. 力扣泡泡龙
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (Vec<i32>, Vec<i32>, usize, bool) {
        if let Some(r) = root {
            let rb = r.borrow();
            let (mut lnohit, mut lhit, mut lcheck, mut lnull) = Self::dfs(&rb.left);
            let (mut rnohit, mut rhit, mut rcheck, mut rnull) = Self::dfs(&rb.right);
            if lnohit.is_empty() && rnohit.is_empty() {
                (vec![rb.val], vec![rb.val], 0, true)
            } else if lnohit.is_empty() || rnohit.is_empty() {
                let (mut nohit, mut hit, check, _) = if lnohit.is_empty() {
                    (rnohit, rhit, rcheck, rnull)
                } else {
                    (lnohit, lhit, lcheck, lnull)
                };
                nohit.push(rb.val);
                hit.push(rb.val);
                let lhit = hit.len();
                for i in 0..check {
                    hit[lhit - 1 - i] = hit[lhit - 1 - i].max(nohit[lhit - 2 - i]);
                }
                if lhit - 1 - check > 0 {
                    hit[lhit - 1 - check] = hit[lhit - 1 - check].max(nohit[lhit - 2 - check]);
                }
                (nohit, hit, 0, true)
            } else {
                if lnohit.len() < rnohit.len() {
                    std::mem::swap(&mut lnohit, &mut rnohit);
                    std::mem::swap(&mut lhit, &mut rhit);
                    std::mem::swap(&mut lcheck, &mut rcheck);
                    std::mem::swap(&mut lnull, &mut rnull);
                }
                for i in 0..rnohit.len() {
                    let mut hitr = rhit[i];
                    if i == 0 && rnull {
                        hitr = hitr.max(0);
                    }
                    let il = lnohit.len() - rnohit.len() + i;
                    let mut hitl = lhit[il];
                    if il == 0 && lnull {
                        hitl = hitl.max(0);
                    }
                    lhit[il] = (hitl + rnohit[i]).max(hitr + lnohit[il]);
                    lnohit[il] += rnohit[i];
                }
                lnull = (lhit.len() > rhit.len()) && lnull;
                lcheck = lcheck.max(rhit.len()) + 1;
                lnohit.push(rb.val);
                lhit.push(rb.val);
                (lnohit, lhit, lcheck, lnull)
            }
        } else {
            (vec![], vec![], 0, true)
        }
    }
    pub fn get_max_layer_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, hit, _, _) = Self::dfs(&root);
        hit.into_iter().max().unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn get_max_layer_sum() {
        let null = NULL;
        assert_eq!(
            Solution::get_max_layer_sum(TreeNode::from_vec(vec![-10, -8, 3, -5, -2, null, -1])),
            -3
        );
        assert_eq!(
            Solution::get_max_layer_sum(TreeNode::from_vec(vec![6, 0, 3, null, 8])),
            11
        );
        assert_eq!(
            Solution::get_max_layer_sum(TreeNode::from_vec(vec![5, 6, 2, 4, null, null, 1, 3, 5])),
            9
        );
        assert_eq!(Solution::get_max_layer_sum(TreeNode::from_vec(vec![-5, 1, 7])), 8);
    }
}
