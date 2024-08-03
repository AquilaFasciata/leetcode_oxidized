use std::collections::HashMap;

struct Solution {
    _placeholder: i32,
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_dict: HashMap<char, u32> = HashMap::new();
        let mut longest_substr: i32 = 0;
        let mut temp_substr: i32 = 0;

        let s = s.chars();
        for letter in s {
            if char_dict.contains_key(&letter) {
                char_dict.clear();
                if temp_substr > longest_substr {
                    longest_substr = temp_substr;
                }
                temp_substr = 0;
            } else {
                char_dict.insert(letter, 1);
                temp_substr += 1;
            }
        }
        return longest_substr;
    }
}

fn main() {
    let out = Solution::length_of_longest_substring(String::from("pwwkew"));
    println!("{out}");
    println!("Hello, world!");
}
