
from typing import List, Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def deleteNode(self, node):
        """
        :type node: ListNode
        :rtype: void Do not return anything, modify node in-place instead.
        """
        node.val = node.next.val
        node.next = node.next.next


# length of v is at least 2
def helper(v: List[int]) -> ListNode:
    dummy = ListNode(0)
    curr = dummy
    for i in v:
        curr.next = ListNode(i)
        curr = curr.next
    return dummy.next

def get_node_by_val(node: ListNode, val: int) -> ListNode:
    while node and node.val!= val:
        node = node.next
    return node

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