// https://leetcode.com/problems/remove-boxes/
// 546. Remove Boxes
pub struct Solution;
impl Solution {
    fn rm_boxes(
        boxes: &Vec<i32>,
        from: usize,
        mut to: usize,
        mut tail: i32,
        cache: &mut std::collections::HashMap<(usize, usize, i32), i32>,
    ) -> i32 {
        // println!("{from} {to} {tail}");
        if from > to {
            return 0;
        }
        let key = (from, to, tail);
        if let Some(&c) = cache.get(&key) {
            return c;
        }
        while from < to && boxes[to - 1] == boxes[to] {
            to -= 1;
            tail += 1;
        }
        let mut c = (tail + 1) * (tail + 1);
        if from == to {
            return c;
        }
        c += Self::rm_boxes(boxes, from, to - 1, 0, cache);
        for split in (from..to).rev() {
            if boxes[split] == boxes[to] && boxes[split + 1] != boxes[to] {
                c = c.max(
                    Self::rm_boxes(boxes, from, split, tail + 1, cache)
                        + Self::rm_boxes(boxes, split + 1, to - 1, 0, cache),
                );
            }
        }
        cache.insert(key, c);
        c
    }
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        Self::rm_boxes(
            &boxes,
            0,
            boxes.len() - 1,
            0,
            &mut std::collections::HashMap::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_boxes() {
        assert_eq!(
            Solution::remove_boxes(vec![
                1, 2, 2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 1,
                2, 1, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 1, 2, 2,
                1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1,
                1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1
            ]),
            2758
        );
        assert_eq!(Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
        assert_eq!(Solution::remove_boxes(vec![1, 1, 1]), 9);
        assert_eq!(Solution::remove_boxes(vec![1]), 1);
    }
}
