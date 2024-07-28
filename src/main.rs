struct Solution {
    placeholder: i32
}

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
            second_num += l2.val * 10_i32.pow(i);
            i += 1;
            l2 = match l2.next {
                Some(node) => node,
                None => break 'second_loop,
            }
        }

        let sum = first_num + second_num;
        let binding = sum.to_string();
        let sum = binding.as_str();
        let mut head_node: ListNode;
        for num in sum.chars() {
            head_node.next = Some(Box::new(ListNode::new(num as i32)));
            head_node = *head_node.next.clone().unwrap();
        }

        return Some(Box::new(head_node));
    }
}

fn main() {
    let _test: Solution;
    println!("Hello, world!");
}
