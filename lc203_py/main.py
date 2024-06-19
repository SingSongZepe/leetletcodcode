
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


from typing import Optional, List

class Solution:
    def removeElements(self, head: Optional[ListNode], val: int) -> Optional[ListNode]:
        if not head:
            return None
        curr = head
        while curr.next:
            if curr.next.val == val:
                curr.next = curr.next.next
            else:
                curr = curr.next
        return head if head.val != val else head.next


def helper(v: List[int]) -> Optional[ListNode]:
    if not v:
        return None
    head = ListNode(v[0])
    curr = head
    for i in range(1, len(v)):
        curr.next = ListNode(v[i])
        curr = curr.next
    return head

def print_list(head: Optional[ListNode]):
    while head:
        print(head.val, end=' ')
        head = head.next
    print()

def main():
    print('Hello World')

if __name__ == '__main__':
    main()