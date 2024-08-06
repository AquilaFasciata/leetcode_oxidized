use std::{char, collections::HashMap};

struct Solution {
    _placeholder: i32
}

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut weight_map: HashMap<char, i32> = HashMap::with_capacity(26);
        for letter in word.chars() {
            let letter_weight = weight_map.entry(letter).or_insert(0);
            *letter_weight += 1;
        }

        return 0_i32;
    }
}

fn main() {

}
