// https://leetcode.com/problems/most-beautiful-item-for-each-query/
// 2070. Most Beautiful Item for Each Query
pub struct Solution;
impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;
        items.sort_unstable_by_key(|x| x[0]);
        let mut i = 0;
        for j in 1..items.len() {
            if items[j][1] == items[i][1] {
                items[i][1] = items[i][1].max(items[j][1]);
            } else if items[j][1] > items[i][1] {
                i += 1;
                items[i][0] = items[j][0];
                items[i][1] = items[j][1];
            }
        }
        items.truncate(i + 1);
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let i = items.partition_point(|x| x[0] <= q);
            if i == 0 {
                ans.push(0);
            } else {
                ans.push(items[i - 1][1]);
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
    fn test_maximum_beauty() {
        assert_eq!(
            Solution::maximum_beauty(
                vec_vec![
                    [193, 732],
                    [781, 962],
                    [864, 954],
                    [749, 627],
                    [136, 746],
                    [478, 548],
                    [640, 908],
                    [210, 799],
                    [567, 715],
                    [914, 388],
                    [487, 853],
                    [533, 554],
                    [247, 919],
                    [958, 150],
                    [193, 523],
                    [176, 656],
                    [395, 469],
                    [763, 821],
                    [542, 946],
                    [701, 676]
                ],
                vec![
                    885, 1445, 1580, 1309, 205, 1788, 1214, 1404, 572, 1170, 989, 265, 153, 151, 1479, 1180, 875, 276,
                    1584
                ]
            ),
            [962, 962, 962, 962, 746, 962, 962, 962, 946, 962, 962, 919, 746, 746, 962, 962, 962, 919, 962]
        );
        assert_eq!(
            Solution::maximum_beauty(vec_vec![[1, 2], [3, 2], [2, 4], [5, 6], [3, 5]], vec![1, 2, 3, 4, 5, 6]),
            [2, 4, 5, 5, 6, 6]
        );
        assert_eq!(
            Solution::maximum_beauty(vec_vec![[1, 2], [1, 2], [1, 3], [1, 4]], vec![1]),
            [4]
        );
        assert_eq!(Solution::maximum_beauty(vec_vec![[10, 1000]], vec![5]), [0]);
    }
}
