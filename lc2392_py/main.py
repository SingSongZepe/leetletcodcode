from typing import List
from collections import defaultdict, deque


class Solution:
    def buildMatrix(self, k: int, rowConditions: List[List[int]], colConditions: List[List[int]]) -> List[List[int]]:
        
        

        pass


class Solution:
    def buildMatrix(self, k: int, rowConditions: list[list[int]], colConditions: list[list[int]]) -> list[list[int]]:
        # return True if all okay and return False if cycle was found
        def dfs(src, graph, visited, cur_path, res) -> bool:
            if src in cur_path:
                return False  # cycle detected

            if src in visited:
                return True  # all okay, but we've already visited this node

            visited.add(src)
            cur_path.add(src)

            for neighbor in graph[src]:
                if not dfs(neighbor, graph, visited, cur_path, res):  # if any child returns false
                    return False

            cur_path.remove(src)  # backtrack path
            res.append(src)
            return True

        # if there will be cycle - return empty array, in other case return 1d array as described above
        def topo_sort(edges) -> list[int]:
            graph = defaultdict(list)
            for src, dst in edges:
                graph[src].append(dst)

            visited: set[int] = set()
            cur_path: set[int] = set()
            res: list[int] = []

            for src in range(1, k + 1, 1):
                if not dfs(src, graph, visited, cur_path, res):
                    return []

            return res[::-1]  # we will have res as reversed so we need to reverse it one more time

        row_sorting: list[int] = topo_sort(rowConditions)
        col_sorting: list[int] = topo_sort(colConditions)
        if [] in (row_sorting, col_sorting):
            return []

        value_position: dict[int, list[int]] = {
            n: [0, 0] for n in range(1, k + 1, 1)
        }  # element -> [row_index, col_index]
        for ind, val in enumerate(row_sorting):
            value_position[val][0] = ind
        for ind, val in enumerate(col_sorting):
            value_position[val][1] = ind

        res: list[list[int]] = [[0 for _ in range(k)] for _ in range(k)]
        for value in range(1, k + 1, 1):
            row, column = value_position[value]
            res[row][column] = value

        return res

# more better solution
class Solution:
    def buildMatrix(
        self, k: int, rowConditions: List[List[int]], colConditions: List[List[int]]
    ) -> List[List[int]]:
        def f(cond: List[List[int]]):
            g = defaultdict(list)
            indeg = [0] * (k + 1)
            for a, b in cond:
                g[a].append(b)
                indeg[b] += 1
            q = deque([i for i, v in enumerate(indeg[1:], 1) if v == 0])
            res = []
            while q:
                for _ in range(len(q)):
                    i = q.popleft()
                    res.append(i)
                    for j in g[i]:
                        indeg[j] -= 1
                        if indeg[j] == 0:
                            q.append(j)
            return None if len(res) != k else res

        row = f(rowConditions)
        col = f(colConditions)
        if row is None or col is None:
            return []
        ans = [[0] * k for _ in range(k)]
        m = [0] * (k + 1)
        for i, v in enumerate(col):
            m[v] = i
        for i, v in enumerate(row):
            ans[i][m[v]] = v
        return ans



def main():
    print('Hello World')

if __name__ == '__main__':
    main()