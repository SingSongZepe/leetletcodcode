

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

from typing import Optional, List


from collections import deque
class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        def check_cvalue(cvalue: List[int]):
            if len(cvalue) == 1:
                return True
            elif len(cvalue) % 2 == 1:
                return False

            for i in range(len(cvalue) // 2):
                if cvalue[i] != cvalue[-i-1]:
                    return False
            return True


        dq = deque([(root, 0)])

        cdpeth = 0
        cvalue = []

        while dq:
            node, depth = dq.popleft()
            if depth > cdpeth:
                if not check_cvalue(cvalue):
                    return False
                cdpeth += 1
                cvalue.clear()
            cvalue.append(node.val if node else 101)
            if node:
                dq.append((node.left, depth+1))
                dq.append((node.right, depth+1))

        return True

class Solution1:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        def same(p,q) -> bool:
            if not p and not q:
                return True
            if not p or not q:
                return False
            if p.val==q.val and same(p.left,q.right) and same(p.right,q.left):
                return True
            return False
        return same(root.left,root.right)


class Solution2:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        def is_mirror(l, r) -> bool:
            if not l and not r:
                return True
            if not l or not r:
                return False
            if l.val == r.val:
                return is_mirror(l.left, r.right) and is_mirror(l.right, r.left)
            return False
        return is_mirror(root.left, root.right)


def helper(v: List[int]) -> Optional[TreeNode]:
    def build_tree(v: List[int], i: int) -> Optional[TreeNode]:
        if i >= len(v) or v[i] == None:
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