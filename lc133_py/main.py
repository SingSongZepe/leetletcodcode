
from typing import Optional, List

# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []

class Solution:
    def cloneGraph(self, root: Optional['Node']) -> Optional['Node']:
        if root is None:
            return None
        new_root = Node(root.val)
        self.recv(root, new_root, [None for i in range(100)]) # 0 - 99
        return new_root

    def recv(self, root: Optional['Node'], new_root: Optional['Node'], newed_nodes: List[Optional[Node]]):
        if len(root.neighbors) <= len(new_root.neighbors):
            return
        newed_nodes[new_root.val - 1] = new_root
        for neighbor in root.neighbors:
            break_flag = False
            # not append
            for nr in new_root.neighbors:
                if nr.val == neighbor.val:
                    break_flag = True

            if break_flag:
                continue

            # append
            if not newed_nodes[neighbor.val - 1]:
                created_root = Node(neighbor.val)
                created_root.neighbors.append(new_root)
                new_root.neighbors.append(created_root)
                newed_nodes[neighbor.val - 1] = created_root
            else:
                new_root.neighbors.append(newed_nodes[neighbor.val - 1])
            self.recv(neighbor, newed_nodes[neighbor.val - 1], newed_nodes)


class Solution1:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        cloned = {}

        def dfs(node):
            if node not in cloned:
                copy = Node(node.val)
                cloned[node] = copy
            else:
                return cloned[node]

            for nei in node.neighbors:
                copy.neighbors.append(
                    dfs(nei))
            return copy

        return dfs(node) if node else None



class Solution2:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:

        createds = {}

        def recv(node: Optional['Node']):
            if node.val in createds:
                return createds[node.val]
            else:
                created = Node(node.val)
                createds[node.val] = created

            for neighbor in node.neighbors:
                created.neighbors.append(recv(neighbor))

            return created

        return recv(node) if node else None


def helper(neighbors: List[List[int]]) -> Optional[Node]:
    if len(neighbors) == 0:
        return None
    nodes = [Node(i + 1) for i in range(len(neighbors))]
    for i, neighbors in enumerate(neighbors):
        for neighbor in neighbors:
            nodes[i].neighbors.append(nodes[neighbor - 1])
    return nodes[0]

def print_neighbor(node: Optional[Node]):
    if node is None:
        print("node is none")
        return
    print(node.val, "has neighbors:", [n.val for n in node.neighbors])

def main():
    print("Hello World!")


if __name__ == '__main__':
    main()