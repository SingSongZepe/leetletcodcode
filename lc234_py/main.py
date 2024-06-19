
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next



from typing import Optional, List
class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        curr = head
        l = 0
        while curr:
            curr = curr.next
            l += 1

        prev = None
        curr = head

        for _ in range(l // 2):
            temp = curr.next
            curr.next = prev
            prev = curr
            curr = temp

        if l % 2 == 1:
            curr = curr.next

        while prev and curr:
            if prev.val != curr.val:
                return False
            prev = prev.next
            curr = curr.next

        return True



class Solution1:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        stack = []
        curr = head
        while curr:
            stack.append(curr.val)
            curr = curr.next

        while len(stack) > 1:
            if stack.pop(0) != stack.pop():
                return False

        return True


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
    curr = head
    while curr:
        print(curr.val, end=' ')
        curr = curr.next
    print()

def main():
    print('Hello World')

if __name__ == '__main__':
    main()