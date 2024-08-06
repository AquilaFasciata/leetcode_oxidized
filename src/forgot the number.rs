struct Solution {
    _placeholder: i32,
}

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut sum_vec: Vec<i32> = nums.clone();
        assert_eq!(n as usize, nums.len());
        for i in 1..n - 1 {
            for j in i..n {
                sum_vec.push(nums[i as usize] + nums[j as usize]);
            }
        }

        sum_vec.sort_unstable();

        println!("{sum_vec:?}");

        return 0_i32;
    }
}

fn main() {
    let out = Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 1);
    println!("{out}");
    println!("Hello, world!");
}
