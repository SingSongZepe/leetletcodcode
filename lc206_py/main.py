
from typing import Optional, List

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head:
            return None
        prev = None 
        curr = head
        while curr:
            next_node = curr.next
            curr.next = prev
            prev = curr
            curr = next_node
        return prev


def build_list(nums: List[int]) -> Optional[ListNode]:
    if not nums:
        return None
    head = ListNode(nums[0])
    curr = head
    for num in nums[1:]:
        curr.next = ListNode(num)
        curr = curr.next
    return head

def print_list(head: Optional[ListNode]) -> None:
    while head:
        print(head.val, end='->')
        head = head.next
    print('None')


def main():
    print('Hello World')

if __name__ == '__main__':
    main()