
# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

from typing import Optional, List

# dfs solution
class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        def dfs(node: Optional[TreeNode], depth: int) -> int:
            if not node:
                return depth - 1
            return max(dfs(node.left, depth+1), dfs(node.right, depth+1))
        return dfs(root, 1)

# bfs solution
class Solution1:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        queue = [(root, 1)]
        while queue:
            node, depth = queue.pop(0)
            if node.left:
                queue.append((node.left, depth+1))
            if node.right:
                queue.append((node.right, depth+1))
        return depth

def helper(v: List[int]) -> Optional[TreeNode]:
    def build_tree(v: List[int], i: int) -> Optional[TreeNode]:
        if i >= len(v) or v[i] == None:
            return None
        root = TreeNode(v[i])
        root.left = build_tree(v, 2*i+1)
        root.right = build_tree(v, 2*i+2)
        return root
    return build_tree(v, 0)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()