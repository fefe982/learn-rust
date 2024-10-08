// https://leetcode.com/problems/find-number-of-coins-to-place-in-tree-nodes/
// 2973. Find Number of Coins to Place in Tree Nodes
pub struct Solution;
impl Solution {
    fn merge<T>(v1: Vec<T>, v2: Vec<T>, cnt: usize) -> Vec<T>
    where
        T: Ord + Copy,
    {
        let mut res = vec![];
        let mut n1 = v1.get(0);
        let mut n2 = v2.get(0);
        let mut i = 0;
        let mut j = 0;
        while res.len() < cnt {
            match (n1, n2) {
                (Some(&a), Some(&b)) => {
                    if a >= b {
                        res.push(a);
                        i += 1;
                        n1 = v1.get(i);
                    } else {
                        res.push(b);
                        j += 1;
                        n2 = v2.get(j);
                    }
                }
                (None, Some(&b)) => {
                    res.push(b);
                    j += 1;
                    n2 = v2.get(j);
                }
                (Some(&a), None) => {
                    res.push(a);
                    i += 1;
                    n1 = v1.get(i);
                }
                (None, None) => break,
            }
        }
        res
    }
    fn walk(
        g: &Vec<Vec<usize>>,
        cost: &Vec<i32>,
        node: usize,
        parent: usize,
        val: &mut Vec<i64>,
    ) -> (Vec<i32>, Vec<std::cmp::Reverse<i32>>) {
        let mut vtop = vec![cost[node]];
        let mut vbot = vec![std::cmp::Reverse(cost[node])];
        for &n in &g[node] {
            if n != parent {
                let (ctop, cbot) = Self::walk(g, cost, n, node, val);
                vtop = Self::merge(vtop, ctop, 3);
                vbot = Self::merge(vbot, cbot, 2);
            }
        }
        if vtop.len() == 3 {
            val[node] =
                (vtop[0] as i64 * (vtop[1] as i64 * vtop[2] as i64).max(vbot[0].0 as i64 * vbot[1].0 as i64)).max(0);
        } else {
            val[node] = 1;
        }
        (vtop, vbot)
    }
    pub fn placed_coins(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64> {
        let mut g = vec![vec![]; cost.len()];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut val = vec![0; cost.len()];
        Self::walk(&g, &cost, 0, usize::MAX, &mut val);
        val
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_placed_coins() {
        assert_eq!(
            Solution::placed_coins(
                vec_vec![
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [0, 5],
                    [0, 6],
                    [0, 7],
                    [0, 8],
                    [0, 9],
                    [0, 10],
                    [0, 11],
                    [0, 12],
                    [0, 13],
                    [0, 14],
                    [0, 15],
                    [0, 16],
                    [0, 17],
                    [0, 18],
                    [0, 19],
                    [0, 20],
                    [0, 21],
                    [0, 22],
                    [0, 23],
                    [0, 24],
                    [0, 25],
                    [0, 26],
                    [0, 27],
                    [0, 28],
                    [0, 29],
                    [0, 30],
                    [0, 31],
                    [0, 32],
                    [0, 33],
                    [0, 34],
                    [0, 35],
                    [0, 36],
                    [0, 37],
                    [0, 38],
                    [0, 39],
                    [0, 40],
                    [0, 41],
                    [0, 42],
                    [0, 43],
                    [0, 44],
                    [0, 45],
                    [0, 46],
                    [0, 47],
                    [0, 48],
                    [0, 49],
                    [0, 50],
                    [0, 51],
                    [0, 52],
                    [0, 53],
                    [0, 54],
                    [0, 55],
                    [0, 56],
                    [0, 57],
                    [0, 58],
                    [0, 59],
                    [0, 60],
                    [0, 61],
                    [0, 62],
                    [0, 63],
                    [0, 64],
                    [0, 65],
                    [0, 66],
                    [0, 67],
                    [0, 68],
                    [0, 69],
                    [0, 70],
                    [0, 71],
                    [0, 72],
                    [0, 73],
                    [0, 74],
                    [0, 75],
                    [0, 76],
                    [0, 77],
                    [0, 78],
                    [0, 79],
                    [0, 80],
                    [0, 81],
                    [0, 82],
                    [0, 83],
                    [0, 84],
                    [0, 85],
                    [0, 86],
                    [0, 87],
                    [0, 88],
                    [0, 89],
                    [0, 90],
                    [0, 91],
                    [0, 92],
                    [0, 93],
                    [0, 94],
                    [0, 95],
                    [0, 96],
                    [0, 97],
                    [0, 98],
                    [0, 99]
                ],
                vec![
                    -5959, 602, -6457, 7055, -1462, 6347, 7226, -8422, -6088, 2997, -7909, 6433, 5217, 3294, -3792,
                    7463, 8538, -3811, 5009, 151, 5659, 4458, -1702, -1877, 2799, 9861, -9668, -1765, 2181, -8128,
                    7046, 9529, 6202, -8026, 6464, 1345, 121, 1922, 7274, -1227, -9914, 3025, 1046, -9368, -7368, 6205,
                    -6342, 8091, -6732, -7620, 3276, 5136, 6871, 4823, -1885, -4005, -3974, -2725, -3845, -8508, 7201,
                    -9566, -7236, -3386, 4021, 6793, -8759, 5066, 5879, -5171, 1011, 1242, 8536, -8405, -9646, -214,
                    2251, -9934, -8820, 6206, 1006, 1318, -9712, 7230, 5608, -4601, 9185, 346, 3056, 8913, -2454,
                    -3445, -4295, 4802, -8852, -6121, -4538, -5580, -9246, -6462
                ]
            ),
            [
                971167251036,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1
            ]
        );
        assert_eq!(
            Solution::placed_coins(vec_vec![[0, 1], [0, 2], [0, 3], [0, 4], [0, 5]], vec![1, 2, 3, 4, 5, 6]),
            [120, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            Solution::placed_coins(
                vec_vec![[0, 1], [0, 2], [1, 3], [1, 4], [1, 5], [2, 6], [2, 7], [2, 8]],
                vec![1, 4, 2, 3, 5, 7, 8, -4, 2]
            ),
            [280, 140, 32, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            Solution::placed_coins(vec_vec![[0, 1], [0, 2]], vec![1, 2, -2]),
            [0, 1, 1]
        );
    }
}
