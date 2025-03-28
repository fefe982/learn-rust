// https://leetcode.com/problems/flatten-nested-list-iterator/
// 341. Flatten Nested List Iterator
pub struct Solution;
use super::nested_integer::NestedInteger;
pub struct NestedIterator {
    vec: Vec<i32>,
    index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn flatten(nested_list: &Vec<NestedInteger>, vec: &mut Vec<i32>) {
        for nested_integer in nested_list {
            match nested_integer {
                NestedInteger::Int(i) => vec.push(*i),
                NestedInteger::List(list) => Self::flatten(list, vec),
            }
        }
    }
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut vec = Vec::new();
        Self::flatten(&nested_list, &mut vec);
        Self { vec, index: 0 }
    }

    pub fn next(&mut self) -> i32 {
        let r = self.vec[self.index];
        self.index += 1;
        r
    }

    pub fn has_next(&self) -> bool {
        self.index < self.vec.len()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
#[cfg(test)]
mod tests {
    use super::*;
    fn flatten(mut iter: NestedIterator) -> Vec<i32> {
        let mut res = vec![];
        while iter.has_next() {
            res.push(iter.next());
        }
        res
    }
    #[test]
    fn test_iter() {
        assert_eq!(
            flatten(NestedIterator::new(vec![
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
                NestedInteger::Int(2),
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            ])),
            vec![1, 1, 2, 1, 1]
        );
        assert_eq!(
            flatten(NestedIterator::new(vec![NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::List(vec![
                    NestedInteger::Int(4),
                    NestedInteger::List(vec![NestedInteger::Int(6)])
                ]),
            ]),])),
            vec![1, 4, 6]
        );
    }
}
