

from typing import List
from collections import defaultdict
class Solution:
    def maximumInvitations(self, favorite: List[int]) -> int:
        depth = []
        pointers = defaultdict(list)
        for i in range(len(favorite)):
            d = 1
            j = favorite[i]

            visited = {i: 0}
            while favorite[j] not in visited:
                visited[j] = d
                d += 1
                j = favorite[j]
            d -= favorite[j]

            pointers[favorite[i]].append(i)
            depth.append(d)
        print(pointers)
        print(depth)

        ml = 0
        for p in pointers:
            prev_max = 0
            for i in range(len(pointers[p])):
                if depth[pointers[p][i]] > 2: # circle
                    ml = max(ml, depth[pointers[p][i]])
                else:
                    ml = max(ml, depth[pointers[p][i]] + prev_max)
                    prev_max = max(prev_max, depth[pointers[p][i]])

        print(ml)

        pass


class Solution1:
    def maximumInvitations(self, favorite: List[int]) -> int:
        ans = 0
        n = len(favorite)
        cycles_of_2 = []
        seen = [False for _ in range(n)]
        for node in range(n):
            if seen[node]:
                continue
            if favorite[favorite[node]] == node:
                cycles_of_2.append((node, favorite[node]))
                seen[node] = True
                seen[favorite[node]] = True

        for node in range(n):
            if seen[node]:
                continue
            cycle_length = self.find_cycle(node, 1, {}, seen, favorite)
            ans = max(ans, cycle_length)

        if len(cycles_of_2) != 0:
            revfavorite = defaultdict(list)
            for node, fav in enumerate(favorite):
                if favorite[fav] != node:
                    revfavorite[fav].append(node)
            total_tail = 0
            for x, y in cycles_of_2:
                max_tail = self.max_depth(x, revfavorite) + self.max_depth(y, revfavorite)
                total_tail += max_tail
            ans = max(ans, 2 * len(cycles_of_2) + total_tail)
        return ans

    def find_cycle(self, node, length, path, seen, favorite):
        if seen[node]:
            return -1
        seen[node] = True
        path[node] = length
        if favorite[node] in path:
            return path[node] - path[favorite[node]] + 1
        return self.find_cycle(favorite[node], length + 1, path, seen, favorite)

    def max_depth(self, node, graph, depth=0):
        res = depth
        for neigh in graph[node]:
            res = max(res, self.max_depth(neigh, graph, depth + 1))
        return res


def main():
    print('Hello World')

if __name__ == '__main__':
    main()