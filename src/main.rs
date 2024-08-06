use std::{char, collections::HashMap};

struct Solution {
    _placeholder: i32
}

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut result: i32 = 0;
        let mut weight_map: HashMap<char, i32> = HashMap::with_capacity(26);
        for letter in word.chars() {
            let letter_weight = weight_map.entry(letter).or_insert(0);
            *letter_weight += 1;
        }

        // TODO Sort weight map

        let mut keypad: Vec<Vec<char>> = vec![Vec::new(); 8];

        for letter in weight_map {
            let key_ind: usize = letter.1 as usize % keypad.len();
            let key = keypad.get_mut(key_ind).unwrap();
            key.push(letter.0);
            result += key.len() as i32;
        }

        return result;
    }
}

fn main() {

}
