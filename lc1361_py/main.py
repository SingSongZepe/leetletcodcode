
from typing import List
class Solution:
    def validateBinaryTreeNodes(self, n: int, leftChild: List[int], rightChild: List[int]) -> bool:
        s = set()

        for i in range(n):
            if leftChild[i]!= -1:
                if leftChild[i] in s:
                    return False
                else:
                    s.add(leftChild[i])

            if rightChild[i]!= -1:
                if rightChild[i] in s:
                    return False
                else:
                    s.add(rightChild[i])

        return True if len(s) == n - 1 else False

class Solution1:
    def validateBinaryTreeNodes(self, n: int, leftChild: List[int], rightChild: List[int]) -> bool:
        def circles(parents):
            for start in range(n):
                if parents[start] == -1:
                    continue
                i = start
                while parents[i] != -1:
                    i = parents[i]
                    if i == start:
                        return True
            return False


        parents = [-1] * n
        for i in range(n):
            if leftChild[i] != -1:
                if parents[leftChild[i]] != -1 or parents[i] == leftChild[i]:
                    return False
                parents[leftChild[i]] = i
            if rightChild[i] != -1:
                if parents[rightChild[i]] != -1 or parents[i] == rightChild[i]:
                    return False
                parents[rightChild[i]] = i
        return True if sum(parent != -1 for parent in parents) == n - 1 and not circles(parents) else False

from collections import deque
class Solution2:
    def validateBinaryTreeNodes(self, n: int, leftChild: List[int], rightChild: List[int]) -> bool:
        count = 0
        indegree = [0] * n
        for i in range(n):
            l = leftChild[i]
            r = rightChild[i]
            if l != -1:
                indegree[l] += 1
            if r != -1:
                indegree[r] += 1
        if indegree.count(0) != 1:
            return False

        root = indegree.index(0)
        q = deque()
        q.append(root)
        while q:
            cur = q.popleft()
            count += 1
            if count > n:
                break
            if leftChild[cur] != -1:
                q.append(leftChild[cur])
            if rightChild[cur] != -1:
                q.append(rightChild[cur])
        return count == n

def main():
    print('Hello World')

if __name__ == '__main__':
    main()