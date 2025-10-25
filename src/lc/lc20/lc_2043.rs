// https://leetcode.com/problems/simple-bank-system/
// 2043. Simple Bank System
pub struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    pub fn new(balance: Vec<i64>) -> Self {
        Bank { balance }
    }

    pub fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let a1 = (account1 - 1) as usize;
        let a2 = (account2 - 1) as usize;
        if a1 >= self.balance.len() || a2 >= self.balance.len() {
            return false;
        }
        if self.balance[a1] < money {
            return false;
        }
        self.balance[a1] -= money;
        self.balance[a2] += money;
        true
    }

    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        let a = (account - 1) as usize;
        if a >= self.balance.len() {
            return false;
        }
        self.balance[a] += money;
        true
    }

    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let a = (account - 1) as usize;
        if a >= self.balance.len() {
            return false;
        }
        if self.balance[a] < money {
            return false;
        }
        self.balance[a] -= money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bank() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        assert_eq!(bank.withdraw(3, 10), true);
        assert_eq!(bank.transfer(5, 1, 20), true);
        assert_eq!(bank.deposit(5, 20), true);
        assert_eq!(bank.transfer(3, 4, 15), false);
        assert_eq!(bank.withdraw(10, 50), false);
    }
}
