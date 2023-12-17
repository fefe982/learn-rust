// https://leetcode.com/problems/design-a-food-rating-system/
// 2353. Design a Food Rating System
struct FoodRatings {
    rating: std::collections::HashMap<String, (i32, String)>,
    high: std::collections::HashMap<
        String,
        std::rc::Rc<std::cell::RefCell<std::collections::BinaryHeap<(i32, std::cmp::Reverse<String>)>>>,
    >,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut rating = std::collections::HashMap::new();
        let mut high = std::collections::HashMap::new();
        for (food, (cusine, rate)) in foods.into_iter().zip(cuisines.into_iter().zip(ratings.into_iter())) {
            rating.insert(food.clone(), (rate, cusine.clone()));
            high.entry(cusine)
                .or_insert(std::rc::Rc::new(std::cell::RefCell::new(
                    std::collections::BinaryHeap::new(),
                )))
                .borrow_mut()
                .push((rate, std::cmp::Reverse(food)));
        }
        Self { rating, high }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let (rating, cusine) = self.rating.get_mut(&food).unwrap();
        let old_rating = *rating;
        *rating = new_rating;
        if old_rating == new_rating {
            return;
        }
        let high = self.high.get(cusine).unwrap().clone();
        let mut high = high.borrow_mut();
        while let Some((r, f)) = high.pop() {
            if f.0 == food {
                continue;
            }
            if self.rating.get(&f.0).unwrap().0 != r {
                continue;
            }
            high.push((r, f));
            break;
        }
        high.push((new_rating, std::cmp::Reverse(food)));
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.high
            .get(&cuisine)
            .unwrap()
            .borrow()
            .peek()
            .map(|(_, f)| f.0.clone())
            .unwrap_or_default()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        let mut food_ratings = FoodRatings::new(
            vec_str!["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"],
            vec_str!["korean", "japanese", "japanese", "greek", "japanese", "korean"],
            vec![9, 12, 8, 15, 14, 7],
        );
        assert_eq!(food_ratings.highest_rated("korean".to_string()), "kimchi");
        assert_eq!(food_ratings.highest_rated("japanese".to_string()), "ramen");
        food_ratings.change_rating("sushi".to_string(), 16);
        assert_eq!(food_ratings.highest_rated("japanese".to_string()), "sushi");
        food_ratings.change_rating("ramen".to_string(), 16);
        assert_eq!(food_ratings.highest_rated("japanese".to_string()), "ramen");
    }
    #[test]
    fn test2() {
        let mut food_ratings = FoodRatings::new(vec_str!["biihw"], vec_str!["okxsrcqn"], vec![13]);
        food_ratings.change_rating("biihw".to_string(), 9);
        food_ratings.change_rating("biihw".to_string(), 6);
    }
}
