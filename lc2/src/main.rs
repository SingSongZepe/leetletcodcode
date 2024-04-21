
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result_front = Box::new(ListNode::new(0));
        let mut current = &mut result_front;
        let mut carry = false;

        let mut num1 = &l1;
        let mut num2 = &l2;

        loop {
            let mut total = 0;

            match (num1, num2) {
                (Some(n1), Some(n2)) => {
                    total = n1.val + n2.val + carry as i32;
                    num1 = &n1.next;
                    num2 = &n2.next;
                }
                (Some(n1), None) => {
                    total = n1.val + carry as i32;
                    num1 = &n1.next;
                }
                (None, Some(n2)) => {
                    total = n2.val + carry as i32;
                    num2 = &n2.next;
                }
                (None, None) => {
                    break;
                }
            }

            if total >= 10 {
                total %= 10;
                carry = true;
            } else {
                carry = false;
            }

            current.next = Some(Box::new(ListNode::new(total)));
            current = current.next.as_mut().unwrap();
        }

        if carry {
            current.next = Some(Box::new(ListNode::new(1)));
        }

        result_front.next
    }
    // pub fn add_two_numbers1(mut l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut current = &mut l1;
    //     let mut carry = false;
    //
    //     let mut num2 = l2;
    //
    //     loop {
    //         let mut total = 0;
    //
    //         match (current, num2) {
    //             (Some(n1), Some(n2)) => {
    //                 total = n1.val + n2.val + if carry { 1 } else { 0 };
    //                 n1.val = total % 10;
    //                 carry = total >= 10;
    //                 current = &mut n1.next;
    //                 num2 = n2.next;
    //             }
    //             (Some(n1), None) => {
    //                 if carry {
    //                     total = n1.val + 1;
    //                     n1.val = total % 10;
    //                     carry = total >= 10;
    //                 } else {
    //                     break;
    //                 }
    //                 current = &mut n1.next;
    //             }
    //             (None, Some(n2)) => {
    //                 if carry {
    //                     total = n2.val + 1;
    //                     let new_node = ListNode {
    //                         val: total % 10,
    //                         next: None,
    //                     };
    //                     *current = Some(Box::new(new_node));
    //                     carry = total >= 10;
    //                 } else {
    //                     break;
    //                 }
    //                 current = &mut num2;
    //             }
    //             (None, None) => {
    //                 if carry {
    //                     *current = Some(Box::new(ListNode::new(1)));
    //                 }
    //                 break;
    //             }
    //         }
    //     }
    //
    //     l1
    // }
    pub fn add_two_numbers2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers2(n1.next, n2.next)
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers2(Solution::add_two_numbers2(carry, n1.next), n2.next)
                    }))
                }
            }
        }
    }
}

fn helper(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut front = Box::new(ListNode::new(0));
    let mut current = &mut front;

    for x in v {
        current.next = Some(Box::new(ListNode::new(x)));
        current = current.next.as_mut().unwrap();
    }

    front.next
}

fn print_list(l: &Option<Box<ListNode>>) {
    let mut current = l;
    loop {
        match current {
            Some(v) => {
                print!("{}", v.val);
                current = &v.next;
            }
            None => {
                println!("end");
                break;
            }
        }
    }
}

fn main() {
    // println!("Hello, world!");
    print_list(&Solution::add_two_numbers2(helper(vec![9,9,9,9,9,9,9]), helper(vec![9, 9, 9, 9])));
}
