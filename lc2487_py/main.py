
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

from typing import Optional, List
class Solution:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0)
        dummy.next = head

        self.remove_next_nodes(dummy, head)

        return dummy.next

    def remove_next_nodes(self, prev: ListNode, curr: Optional[ListNode]) -> None:
        if not curr:
            return

        if self.has_larger_node(curr.next, curr.val):
            prev.next = curr.next
            self.remove_next_nodes(prev, curr.next)
        else:
            self.remove_next_nodes(curr, curr.next)

    def has_larger_node(self, node: ListNode, value: int) -> bool:
        while node:
            if node.val > value:
                return True
            node = node.next
        return False

class Solution1:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0)
        dummy.next = head
        self.recv(dummy)
        return dummy.next

    def recv(self, curr: Optional[ListNode]) -> Optional[ListNode]:
        if curr.next is None:
            return curr
        curr.next = self.recv(curr.next)
        if curr.next.val > curr.val:
            return curr.next
        else:
            return curr


class Solution2:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        return self.recv(head)

    def recv(self, curr: Optional[ListNode]) -> Optional[ListNode]:
        if curr.next is None:
            return curr
        curr.next = self.recv(curr.next)
        if curr.next.val > curr.val:
            return curr.next
        else:
            return curr


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