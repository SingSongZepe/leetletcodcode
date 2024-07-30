
# Definition for a binary tree node.
from typing import List
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    nodes = []
    def balanceBST(self, root: TreeNode) -> TreeNode:
        self.get_sorted_nodes(root)
        return self.make_new_tree(0, len(self.nodes)-1)

    def get_sorted_nodes(self, node: TreeNode):
        if node:
            self.get_sorted_nodes(node.left)
            self.nodes.append(node)
            self.get_sorted_nodes(node.right)

    def make_new_tree(self, start: int, end: int) -> TreeNode:
        if start > end:
            return None
        mid = (start + end) // 2
        node = self.nodes[mid]
        node.left = self.make_new_tree(start, mid-1)
        node.right = self.make_new_tree(mid+1, end)
        return node





def helper(v: List[int]) -> TreeNode:
    def build(node: TreeNode | None, i: int) -> TreeNode:
        if i >= len(v) or v[i] is None:
            return node
        node = TreeNode(v[i])
        node.left = build(node.left, 2*i+1)
        node.right = build(node.right, 2*i+2)
        return node

    return build(None, 0)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()