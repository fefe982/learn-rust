// https://leetcode.com/problems/design-movie-rental-system/
// 1912. Design Movie Rental System
pub struct MovieRentingSystem {
    prices: std::collections::HashMap<i32, std::collections::HashMap<i32, i32>>,
    stock: std::collections::HashMap<i32, std::collections::BTreeSet<(i32, i32)>>,
    rented: std::collections::BTreeSet<(i32, i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    pub fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut prices = std::collections::HashMap::<i32, std::collections::HashMap<i32, i32>>::new();
        let mut stock = std::collections::HashMap::<i32, std::collections::BTreeSet<(i32, i32)>>::new();
        for entry in entries {
            let shop = entry[0];
            let movie = entry[1];
            let price = entry[2];
            prices.entry(movie).or_default().insert(shop, price);
            stock.entry(movie).or_default().insert((price, shop));
        }
        Self {
            prices,
            stock,
            rented: std::collections::BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        let mut ret = Vec::new();
        if let Some(stock) = self.stock.get(&movie) {
            for (_, shop) in stock {
                ret.push(*shop);
                if ret.len() >= 5 {
                    break;
                }
            }
        }
        ret
    }

    pub fn rent(&mut self, shop: i32, movie: i32) {
        let price = self.prices[&movie][&shop];
        self.rented.insert((price, shop, movie));
        self.stock.entry(movie).or_default().remove(&(price, shop));
    }

    pub fn drop(&mut self, shop: i32, movie: i32) {
        let price = self.prices[&movie][&shop];
        self.rented.remove(&(price, shop, movie));
        self.stock.entry(movie).or_default().insert((price, shop));
    }

    pub fn report(&self) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        for (_, shop, movie) in self.rented.iter() {
            ret.push(vec![*shop, *movie]);
            if ret.len() >= 5 {
                break;
            }
        }
        ret
    }
}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        let mut movie_renting_system = MovieRentingSystem::new(
            3,
            vec_vec![[0, 1, 5], [0, 2, 6], [0, 3, 7], [1, 1, 4], [1, 2, 7], [2, 1, 5]],
        );
        assert_eq!(movie_renting_system.search(1), [1, 0, 2]); // return [1, 0, 2], Movies of ID 1 are unrented at shops 1, 0, and 2. Shop 1 is cheapest; shop 0 and 2 are the same price, so order by shop number.
        movie_renting_system.rent(0, 1); // Rent movie 1 from shop 0. Unrented movies at shop 0 are now [2,3].
        movie_renting_system.rent(1, 2); // Rent movie 2 from shop 1. Unrented movies at shop 1 are now [1].
        assert_eq!(movie_renting_system.report(), [[0, 1], [1, 2]]); // return [[0, 1], [1, 2]]. Movie 1 from shop 0 is cheapest, followed by movie 2 from shop 1.
        movie_renting_system.drop(1, 2); // Drop off movie 2 at shop 1. Unrented movies at shop 1 are now [1,2].
        assert_eq!(movie_renting_system.search(2), [0, 1]); // return [0, 1]. Movies of ID 2 are unrented at shops 0 and 1. Shop 0 is cheapest, followed by shop 1.
    }
}
