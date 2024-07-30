
# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

from typing import Optional, List
class Solution:
    def findTarget(self, root: Optional[TreeNode], k: int) -> bool:


        pass

def helper(v: List[int]) -> Optional[TreeNode]:
    def build_tree(node, i) -> Optional[TreeNode]:
        if i >= len(v):
            return None
        if v[i] is None or v[i] == -1:
            return None
        node = TreeNode(v[i])
        node.left = build_tree(node.left, 2*i+1)
        node.right = build_tree(node.right, 2*i+2)
        return node

    return build_tree(None, 0)


def main():
    print('Hello World')

if __name__ == '__main__':
    main()