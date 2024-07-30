mod test;

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

fn helper(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut().unwrap();

    for val in nums {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }

    head.unwrap().next
}

fn print_list(head: &Option<Box<ListNode>>) {
    if head.is_none() {
        println!("None");
        return;
    }
    let mut current = head.as_ref();
    while let Some(node) = current {
        print!("{} ", node.val);
        current = current.unwrap().next.as_ref();
    }
    println!("");
}

struct Solution;

impl Solution {
	pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut stack1 = vec![];
        let mut stack2 = vec![];
        let binding = head.unwrap();
        let mut curr = binding.as_ref();
        loop {
            if curr.val < x {
                stack1.push(curr.val);
            } else {
                stack2.push(curr.val);
            }
            curr = if curr.next.is_none() {
                break;
            } else {
                curr.next.as_ref().unwrap()
            }
        }
        Self::make_list(&stack1, &stack2)
	}
    pub fn make_list(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut().unwrap();
        for &val in nums1 {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        for &val in nums2 {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        head.unwrap().next
    }
}

struct Solution1;

impl Solution1 {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
          return None;
        }
        let mut dummy1 = ListNode::new(0);
        let mut dummy2 = ListNode::new(0);
        let mut head = head.as_mut().unwrap();

        let mut curr1 = &mut dummy1;
        let mut curr2 = &mut dummy2;
        loop {
            if head.val < x {
                curr1.next = Some(Box::new(ListNode::new(head.val)));
                curr1 = curr1.next.as_mut().unwrap();
            } else {
                curr2.next = Some(Box::new(ListNode::new(head.val)));
                curr2 = curr2.next.as_mut().unwrap();
            }
            head = if head.next.is_none() {
                break;
            } else {
                head.next.as_mut().unwrap()
            }
        }
        curr1.next = dummy2.next;
        dummy1.next
    }
}

struct Solution2;

impl Solution2 {
    // attempting to design a recursive solution
    // but it's not working yet
    fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => {
                let mut current = node;
                let mut prev = None;
                let mut new_head = None;

                while let Some(mut next_node) = current.next.take() {
                    if next_node.val < x {
                        current.next = next_node.next.take();
                        next_node.next = prev.take();
                        if new_head.is_none() {
                            new_head = Some(next_node);
                        } else {
                            let mut tail = new_head.as_mut().unwrap();
                            while let Some(ref mut t) = tail.next {
                                tail = t;
                            }
                            tail.next = Some(next_node);
                        }
                    } else {
                        prev = Some(current);
                        current = next_node;
                    }
                }

                if let Some(mut tail) = new_head {
                    while let Some(t) = tail.next {
                        tail = t;
                    }
                    tail.next = Some(current);
                    Some(tail)
                } else {
                    current.next = Self::partition(current.next, x);
                    Some(current)
                }
            }
        }
    }
}

struct Solution3;

impl Solution3 {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut less_dummy =ListNode::new(0);
        let mut greater_or_equal_dummy = ListNode::new(0);
        let mut less_tail = &mut less_dummy;
        let mut greater_or_equal_tail = &mut greater_or_equal_dummy;
        let mut current = head;


        while let Some(mut node) = current.take() {
            current = node.next.take();
            if node.val < x {
                less_tail.next = Some(node);
                less_tail = less_tail.next.as_mut().unwrap();
            } else {
                greater_or_equal_tail.next = Some(node);
                greater_or_equal_tail = greater_or_equal_tail.next.as_mut().unwrap();
            }
        }

        greater_or_equal_tail.next = None;
        less_tail.next = greater_or_equal_dummy.next;
        less_dummy.next
    }
}

fn main() {
    println!("Hello, world!");
}
