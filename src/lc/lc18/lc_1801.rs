// https://leetcode.com/problems/number-of-orders-in-the-backlog/
// 1801. Number of Orders in the Backlog
pub struct Solution;
impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        const MOD: i64 = 1_000_000_007;

        // Max-heap by price for buy orders: (price, amount)
        let mut buy_heap: BinaryHeap<(i32, i64)> = BinaryHeap::new();
        // Min-heap by price for sell orders via Reverse: (price, amount)
        let mut sell_heap: BinaryHeap<Reverse<(i32, i64)>> = BinaryHeap::new();

        for order in orders {
            let price = order[0];
            let mut amount = order[1] as i64;
            let order_type = order[2];

            if order_type == 0 {
                // Match buy with lowest-price sell while possible.
                while amount > 0 {
                    let Some(Reverse((sell_price, sell_amount))) = sell_heap.pop() else {
                        break;
                    };

                    if sell_price > price {
                        sell_heap.push(Reverse((sell_price, sell_amount)));
                        break;
                    }

                    let matched = amount.min(sell_amount);
                    amount -= matched;
                    let remain_sell = sell_amount - matched;
                    if remain_sell > 0 {
                        sell_heap.push(Reverse((sell_price, remain_sell)));
                    }
                }

                if amount > 0 {
                    buy_heap.push((price, amount));
                }
            } else {
                // Match sell with highest-price buy while possible.
                while amount > 0 {
                    let Some((buy_price, buy_amount)) = buy_heap.pop() else {
                        break;
                    };

                    if buy_price < price {
                        buy_heap.push((buy_price, buy_amount));
                        break;
                    }

                    let matched = amount.min(buy_amount);
                    amount -= matched;
                    let remain_buy = buy_amount - matched;
                    if remain_buy > 0 {
                        buy_heap.push((buy_price, remain_buy));
                    }
                }

                if amount > 0 {
                    sell_heap.push(Reverse((price, amount)));
                }
            }
        }

        let mut total = 0i64;
        for (_, amount) in buy_heap {
            total = (total + amount) % MOD;
        }
        for Reverse((_, amount)) in sell_heap {
            total = (total + amount) % MOD;
        }

        total as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn get_number_of_backlog_orders() {
        assert_eq!(
            Solution::get_number_of_backlog_orders(vec_vec![[10, 5, 0], [15, 2, 1], [25, 1, 1], [30, 4, 0]]),
            6
        );
        assert_eq!(
            Solution::get_number_of_backlog_orders(vec_vec![
                [7, 1000000000, 1],
                [15, 3, 0],
                [5, 999999995, 0],
                [5, 1, 1]
            ]),
            999999984
        );
    }
}
