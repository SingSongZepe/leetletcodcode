mod test;

// #[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut v = vec![];
        let binding = head.unwrap();
        let mut curr = binding.as_ref();
        while curr.next.is_some() {
            if curr.val != curr.next.as_ref().unwrap().val {
                v.push(curr.val);
            }
            curr = curr.next.as_ref().unwrap();
        }
        if v.len() == 0 || *v.last().unwrap() != curr.val {
            v.push(curr.val);
        }
        Self::make_list(v)
    }
    fn make_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut().unwrap();
        for val in v {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        head.unwrap().next
    }
}


struct Solution1;

impl Solution1 {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut dummy = ListNode::new(i32::MAX);
        let mut curr = &mut dummy;
        let binding = head.unwrap();
        let mut head = binding.as_ref();
        while head.next.is_some() {
            if head.val != head.next.as_ref().unwrap().val {
                curr.next = Some(Box::new(ListNode::new(head.val)));
                curr = curr.next.as_mut().unwrap();
            }
            head = head.next.as_ref().unwrap();
        }
        if curr.val != head.val {
            curr.next = Some(Box::new(ListNode::new(head.val)));
        }
        dummy.next
    }
}

struct Solution2;

impl Solution2 {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut dummy = ListNode::new(i32::MAX);
        let mut curr = &mut dummy;
        let mut head = head;
        let mut opt_head = head.as_deref();

        while let Some(node) = opt_head {
            if let Some(next_node) = node.next.as_deref() {
                if node.val != next_node.val {
                    let tmp_node = Box::new(ListNode::new(node.val));
                    curr.next = Some(tmp_node);
                    curr = curr.next.as_mut().unwrap();
                }
                opt_head = node.next.as_deref();
            } else {
                break;
            }
        }
        if curr.val != opt_head.unwrap().val {
            curr.next = Some(Box::new(ListNode::new(opt_head.unwrap().val)));
        }
        dummy.next
    }
}


struct Solution3;

impl Solution3 {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => return head,
            Some(mut x) => {
                if let Some(y) = x.next {
                    if y.val == x.val {
                        x.next = y.next;
                        return Solution::delete_duplicates(Some(x));
                    } else {
                        x.next = Solution::delete_duplicates(Some(y));
                        return Some(x);
                    }
                }
                return Some(x);
            }
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

fn main() {
    println!("Hello, world!");
}
