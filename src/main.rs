#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.unwrap();
        let mut l2 = l2.unwrap();
        let mut sum: i32 = 0;
        let mut first_num: i32 = 0;
        let mut second_num: i32 = 0;

        let mut i: u32 = 0;
        'first_loop: loop {
            first_num += l1.val * 10_i32.pow(i);
            i += 1;
            l1 = match l1.next {
                Some(node) => node,
                None => break 'first_loop,
            }
        }

        i = 0;
        'second_loop: loop {
             += l1.val * 10_i32.pow(i);
            i += 1;
            l1 = match l1.next {
                Some(node) => node,
                None => break 'second_loop,
            }
        }
        return None;
    }
}

fn main() {
    println!("Hello, world!");
}
