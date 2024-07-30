
# Definition for a binary tree node.
from typing import Optional, List
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    sum = 0
    def convertBST(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfs(node: Optional[TreeNode]):
            if node:
                dfs(node.right)
                self.sum += node.val
                node.val = self.sum
                dfs(node.left)

        dfs(root)
        return root

def helper(v: List[int]) -> Optional[TreeNode]:
    def build(node: Optional[TreeNode], i: int) -> Optional[TreeNode]:
        if i >= len(v) or v[i] is None:
            return None
        node = TreeNode(v[i])
        node.left = build(node.left, 2*i+1)
        node.right = build(node.right, 2*i+2)
        return node
    return build(None, 0)



def main():
    print('Hello World')

if __name__ == '__main__':
    main()