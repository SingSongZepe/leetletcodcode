
# Definition for a Node.


class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children

class Solution:
    def maxDepth(self, root: 'Node') -> int:
        def dfs(node, depth):
            if not node.children:
                return depth
            return max(dfs(child, depth + 1) for child in node.children)
        return dfs(root, 1)

NE_INFINITE_ = -1
from typing import List
def helper(nodes):
    if not nodes:
        return None

    root = Node(val=int(nodes[0]) if nodes[0] != NE_INFINITE_ else None)
    queue = [root]
    i = 2

    while queue and i < len(nodes):
        node = queue.pop(0)
        children = []

        while i < len(nodes) and nodes[i] != NE_INFINITE_:
            child = Node(val=int(nodes[i]))
            children.append(child)
            queue.append(child)
            i += 1

        i += 1  # Skip the -1 value
        node.children = children

    return root


def main():
    print('Hello World')

if __name__ == '__main__':
    main()