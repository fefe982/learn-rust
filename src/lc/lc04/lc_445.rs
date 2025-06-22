// https://leetcode.com/problems/add-two-numbers-ii/
// 445. Add Two Numbers II
use super::linked_list::ListNode;
pub struct Solution;
impl Solution {
    fn list_to_vec(mut l: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        while let Some(n) = l {
            res.push(n.val);
            l = n.next;
        }
        res
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let lv1 = Self::list_to_vec(l1);
        let lv2 = Self::list_to_vec(l2);
        let mut rv = vec![];
        let mut carry = 0;
        for ((i1, v1), (i2, v2)) in lv1
            .iter()
            .rev()
            .cloned()
            .enumerate()
            .chain(std::iter::repeat((usize::MAX, 0)))
            .zip(
                lv2.iter()
                    .rev()
                    .cloned()
                    .enumerate()
                    .chain(std::iter::repeat((usize::MAX, 0))),
            )
        {
            let d = v1 + v2 + carry;
            if d == 0 && i1 == usize::MAX && i2 == usize::MAX {
                break;
            }
            rv.push(d % 10);
            carry = d / 10;
        }
        let mut head = None;
        for n in rv {
            let mut next = Box::new(ListNode::new(n));
            next.next = head;
            head = Some(next);
        }
        head
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(vec![7, 2, 4, 3]),
                ListNode::from_vec(vec![5, 6, 4])
            ),
            ListNode::from_vec(vec![7, 8, 0, 7])
        );
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(vec![2, 4, 3]),
                ListNode::from_vec(vec![5, 6, 4])
            ),
            ListNode::from_vec(vec![8, 0, 7])
        );
        assert_eq!(
            Solution::add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0])),
            ListNode::from_vec(vec![0])
        );
    }
}
