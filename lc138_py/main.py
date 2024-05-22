
# Definition for a Node.

from typing import Optional, List
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next: Optional['Node'] = next
        self.random: Optional['Node'] = random


class Solution:
    def copyRandomList(self, head: 'Optional[Node]') -> 'Optional[Node]':
        create_dict = {}

        def recv(node: 'Optional[Node]') -> 'Optional[Node]':
            if node in create_dict:
                return create_dict[node]

            if node is None:
                return None

            create_node = Node(node.val)
            create_dict[node] = create_node
            if node.random is not None:
                create_node.random = recv(node.random)
            create_node.next = recv(node.next)
            return create_node

        return recv(head) if head is not None else None

class Solution1:
    def copyRandomList(self, head: 'Optional[Node]') -> 'Optional[Node]':
        if not head:
            return None

        curr = head
        while curr:
            new_node = Node(curr.val, curr.next)
            curr.next = new_node
            curr = new_node.next

        curr = head
        while curr:
            if curr.random:
                curr.next.random = curr.random.next
            curr = curr.next.next

        old_head = head
        new_head = head.next
        curr_old = old_head
        curr_new = new_head

        while curr_old:
            curr_old.next = curr_old.next.next
            curr_new.next = curr_new.next.next if curr_new.next else None
            curr_old = curr_old.next
            curr_new = curr_new.next

        return new_head

def helper(vec: List[List[Optional[int]]]) -> 'Optional[Node]':
    nodes = [Node(0) for i in range(len(vec))]
    for idx, n in enumerate(vec):
        nodes[idx].val = n[0]
        nodes[idx].next = nodes[idx + 1] if idx + 1 < len(vec) else None
        if n[1] is not None:
            nodes[idx].random = nodes[n[1]]
    return nodes[0]

def print_list(head: 'Optional[Node]'):
    while head is not None:
        print(head.val, head.random.val if head.random is not None else None)
        head = head.next
    print()

def main():
    print('Hello World')

if __name__ == '__main__':
    main()