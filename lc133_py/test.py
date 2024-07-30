import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        adjList = [[2, 4], [1, 3], [2, 4], [1, 3]]
        root = helper(adjList)
        print_neighbor(root)
        print_neighbor(root.neighbors[0])
        print_neighbor(root.neighbors[0].neighbors[1])
        print_neighbor(root.neighbors[1])
        result = Solution().cloneGraph(root)
        print_neighbor(result)
        print_neighbor(result.neighbors[0])
        print_neighbor(result.neighbors[0].neighbors[1])
        print_neighbor(result.neighbors[1])

    def test2(self):
        adjList = [[]]
        root = helper(adjList)
        print_neighbor(root)
        result = Solution().cloneGraph(root)
        print_neighbor(result)

    def test3(self):
        adjList = []
        root = helper(adjList)
        print_neighbor(root)
        result = Solution().cloneGraph(root)
        print_neighbor(result)

    # solution 1
    def test11(self):
        adjList = [[2, 4], [1, 3], [2, 4], [1, 3]]
        root = helper(adjList)
        print_neighbor(root)
        print_neighbor(root.neighbors[0])
        print_neighbor(root.neighbors[0].neighbors[1])
        print_neighbor(root.neighbors[1])
        result = Solution1().cloneGraph(root)
        print_neighbor(result)
        print_neighbor(result.neighbors[0])
        print_neighbor(result.neighbors[0].neighbors[1])
        print_neighbor(result.neighbors[1])

    def test21(self):
        adjList = [[]]
        root = helper(adjList)
        print_neighbor(root)
        result = Solution1().cloneGraph(root)
        print_neighbor(result)

    def test31(self):
        adjList = []
        root = helper(adjList)
        print_neighbor(root)
        result = Solution1().cloneGraph(root)
        print_neighbor(result)


    # solution 2
    def test12(self):
        adjList = [[2, 4], [1, 3], [2, 4], [1, 3]]
        root = helper(adjList)
        print_neighbor(root)
        print_neighbor(root.neighbors[0])
        print_neighbor(root.neighbors[0].neighbors[1])
        print_neighbor(root.neighbors[1])
        result = Solution2().cloneGraph(root)
        print_neighbor(result)
        print_neighbor(result.neighbors[0])
        print_neighbor(result.neighbors[0].neighbors[1])
        print_neighbor(result.neighbors[1])

    def test22(self):
        adjList = [[]]
        root = helper(adjList)
        print_neighbor(root)
        result = Solution2().cloneGraph(root)
        print_neighbor(result)

    def test32(self):
        adjList = []
        root = helper(adjList)
        print_neighbor(root)
        result = Solution2().cloneGraph(root)
        print_neighbor(result)

    # def test_array(self):
    #     arr = []
    #     arr[100] = True
    #     print(arr)


if __name__ == '__main__':
    unittest.main()

