use std::{cmp::max, collections::HashSet};

struct Solution {
    _placeholder: i32,
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set: HashSet<char> = HashSet::new();
        let mut left_ind = 0;
        //right_ind is the letter.0 in the enumerated for loop
        let mut result = 0;

        for letter in s.clone().chars().enumerate() {
            loop {
                if !char_set.insert(letter.1) {
                    char_set.remove(&s.chars().nth(left_ind as usize).unwrap());
                    left_ind += 1;
                } else {
                    break;
                }
            }
            result = max(result, (letter.0 as i32) - left_ind + 1);
        }
        return result;
    }
}

fn main() {
    let out = Solution::length_of_longest_substring(String::from("pwwkew"));
    println!("{out}");
    println!("Hello, world!");
}
