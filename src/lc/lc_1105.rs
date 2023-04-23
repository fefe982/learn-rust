// https://leetcode.com/problems/filling-bookcase-shelves/
// 1105. Filling Bookcase Shelves
pub struct Solution;
impl Solution {
    fn min_height_recuse(
        books: &Vec<Vec<i32>>,
        shelf_width: i32,
        start: usize,
        cache: &mut Vec<i32>,
    ) -> i32 {
        if start == books.len() {
            return 0;
        }
        if cache[start] > 0 {
            return cache[start];
        }
        let mut this_height = 0;
        let mut this_width = 0;
        let mut min_height = i32::MAX;
        for i in start..books.len() {
            this_width += books[i][0];
            if this_width > shelf_width {
                break;
            }
            this_height = std::cmp::max(this_height, books[i][1]);
            min_height = std::cmp::min(
                min_height,
                this_height + Self::min_height_recuse(books, shelf_width, i + 1, cache),
            )
        }
        cache[start] = min_height;
        min_height
    }
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        Self::min_height_recuse(&books, shelf_width, 0, &mut vec![0; books.len()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;
    #[test]
    fn min_height_shelves() {
        assert_eq!(
            Solution::min_height_shelves(
                vec_vec![[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]],
                4
            ),
            6
        );
        assert_eq!(
            Solution::min_height_shelves(vec_vec![[1, 3], [2, 4], [3, 2]], 6),
            4
        );
    }
}
