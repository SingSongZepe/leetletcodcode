
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

from typing import Optional, List
class Solution:
    def doubleIt(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(1 if 2 * head.val > 9 else 0)
        dummy.next = head

        while head.next: # not process last node
            head.val = (2 * head.val + (1 if 2 * head.next.val > 9 else 0)) % 10
            head = head.next

        head.val = 2 * head.val % 10

        return dummy if dummy.val == 1 else dummy.next

def helper(v: List[int]) -> ListNode:
    dummy = ListNode(0)
    curr = dummy
    for i in v:
        curr.next = ListNode(i)
        curr = curr.next
    return dummy.next

def print_list(head: ListNode) -> None:
    curr = head
    while curr:
        print(curr.val, end='->')
        curr = curr.next
    print()


def main():
    print('Hello World')

if __name__ == '__main__':
    main()