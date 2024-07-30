
# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

from typing import Optional
from collections import deque
class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        dq = deque()

        def dfs(node):
            if node is None:
                return
            if node.left is None and node.right is None:
                dq.append(node.val)
            else:
                dfs(node.left)
                dfs(node.right)

        def dfs_check(node) -> bool:
            if node is None:
                return True
            if node.left is None and node.right is None:
                if len(dq) == 0 or node.val != dq.popleft():
                    return False
                return True
            else:
                return dfs_check(node.left) and dfs_check(node.right)

        dfs(root1)
        result = dfs_check(root2)
        return result

class Solution1:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        def dfs(node, v):
            if node is None:
                return
            if node.left is None and node.right is None:
                v.append(node.val)
            else:
                dfs(node.left, v)
                dfs(node.right, v)

        v1, v2 = [], []
        dfs(root1, v1)
        dfs(root2, v2)
        return v1 == v2

def helper(v) -> Optional[TreeNode]:
    if v is None:
        return None
    def build_tree(v, i):
        if i >= len(v):
            return None
        if v[i] is None or v[i] == -1:
            return None
        node = TreeNode(v[i])
        node.left = build_tree(v, 2*i+1)
        node.right = build_tree(v, 2*i+2)
        return node
    return build_tree(v, 0)



def main():
    print('Hello World')

if __name__ == '__main__':
    main()