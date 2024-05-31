
NE_INFINITY_ = -100001

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

from typing import Optional, List


from collections import deque
class Solution:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root1:
            return root2

        queue = deque([(root1, root2)])
        while queue:
            node1, node2 = queue.popleft()
            if not node1 or not node2:
                continue
            node1.val = node1.val + node2.val
            if not node1.left:
                node1.left = node2.left
            else:
                queue.append((node1.left, node2.left))
            if not node1.right:
                node1.right = node2.right
            else:
                queue.append((node1.right, node2.right))
        return root1


class Solution1:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root1:
            return root2
        if not root2:
            return root1
        root1.val += root2.val
        root1.left = self.mergeTrees(root1.left, root2.left)
        root1.right = self.mergeTrees(root1.right, root2.right)
        return root1

def helper(v: List[int]) -> Optional[TreeNode]:
    def build_tree(node, idx) -> Optional[TreeNode]:
        if idx < len(v) and v[idx] != NE_INFINITY_:
            node = TreeNode(v[idx])
            node.left = build_tree(node.left, 2*idx+1)
            node.right = build_tree(node.right, 2*idx+2)
            return node
        return None
    return build_tree(None, 0)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()