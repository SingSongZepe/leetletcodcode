### Process with ListNode

how to do this, given two ListNode

> list1: Option<Box<ListNode>>
> list2: Option<Box<ListNode>>,

and then iterate them, when list1.val < list2.val

we do list1 = list1.next, otherwise do list2 = list2.next

if we want to record them to a new list

> curr

which type you select to be?

> Option<&mut Box<ListNode>>
> 
> &mut Box<ListNode>
> 
> &mut ListNode

we can select &mut ListNode as the result

```rust
    curr.next = list1.take();
    curr = curr.next.as_mut().unwrap();
    list1 = curr.next.take();
```

the ownership of the list1 pass to current.next
and then pass the mutable ref to curr.next
and then pass the ownership to list1

