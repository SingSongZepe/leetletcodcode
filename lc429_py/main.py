
from typing import List, Optional

# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children

# BFS solution
class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        
        stack, res = [root], []
        while stack:
            level = []
            for i in range(len(stack)):
                node = stack.pop(0)
                if node:
                    level.append(node.val)
                    stack.extend(node.children)
            if level:
                res.append(level)

        return res
    

# DFS solution
class Solution1:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        res = []

        def dfs(node, level):
            if not node:
                return
            if len(res) < level+1:
                res.append([])

            res[level].append(node.val)
            for child in node.children:
                dfs(child, level+1)

        dfs(root, 0)
        return res
            

# binary tree building function
def build_tree(arr: List[int]) -> Optional[Node]:
    if not arr:
        return None
    def build(arr: List[int], i: int) -> Node:
        if i >= len(arr) or arr[i] is None:
            return None
        node = Node(arr[i])
        node.children = [build(arr, 2*i+1), build(arr, 2*i+2)]
        return node
    return build(arr, 0)

def main():
    print('Hello World')

if __name__ == '__main__':
    main()