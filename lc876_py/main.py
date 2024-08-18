

from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        p1 = p2 = head

        while p2.next:
            if p2.next.next:
                p1 = p1.next
                p2 = p2.next.next
            else:
                return p1.next
        
        return p1
    

def build_list(nums):
    if not nums:
        return None
    head = ListNode(nums[0])
    p = head
    for i in range(1, len(nums)):
        p.next = ListNode(nums[i])
        p = p.next
    return head

def print_list(head):
    p = head
    while p:
        print(p.val, end='->')
        p = p.next
    print()



def main():
    print('Hello World')

if __name__ == '__main__':
    main()