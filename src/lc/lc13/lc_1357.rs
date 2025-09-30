// https://leetcode.com/problems/apply-discount-every-n-orders/
// 1357. Apply Discount Every N Orders
pub struct Cashier {
    n: i32,
    discount: i32,
    product_price: std::collections::HashMap<i32, i32>,
    order_count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {
    pub fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let product_price = products
            .into_iter()
            .zip(prices.into_iter())
            .collect::<std::collections::HashMap<_, _>>();
        Self {
            n,
            discount,
            product_price,
            order_count: 0,
        }
    }

    pub fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.order_count += 1;
        let mut total = 0.0;
        for (p, a) in product.into_iter().zip(amount.into_iter()) {
            total += self.product_price[&p] as f64 * a as f64;
        }
        if self.order_count % self.n == 0 {
            total *= (100 - self.discount) as f64 / 100.0;
        }
        total
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn test_1357() {
        let mut cashier = Cashier::new(
            3,
            50,
            vec![1, 2, 3, 4, 5, 6, 7],
            vec![100, 200, 300, 400, 300, 200, 100],
        );
        assert_approx_eq!(cashier.get_bill(vec![1, 2], vec![1, 2]), 500.0, 1e-5);
        assert_approx_eq!(cashier.get_bill(vec![3, 7], vec![10, 10]), 4000.0, 1e-5);
        assert_approx_eq!(
            cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]),
            800.0,
            1e-5
        );
        assert_approx_eq!(cashier.get_bill(vec![4], vec![10]), 4000.0, 1e-5);
        assert_approx_eq!(cashier.get_bill(vec![7, 3], vec![10, 10]), 4000.0, 1e-5);
        assert_approx_eq!(
            cashier.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]),
            7350.0,
            1e-5
        );
        assert_approx_eq!(cashier.get_bill(vec![2, 3, 5], vec![5, 3, 2]), 2500.0, 1e-5);
    }
}
