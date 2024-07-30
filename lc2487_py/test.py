import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        v = [5, 2, 13, 3, 8]
        head = helper(v)
        result = Solution().removeNodes(head)
        print_list(result)

    def test2(self):
        v = [1, 1, 1, 1]
        head = helper(v)
        result = Solution().removeNodes(head)
        print_list(result)


    def test11(self):
        v = [5, 2, 13, 3, 8]
        head = helper(v)
        result = Solution1().removeNodes(head)
        print_list(result)

    def test21(self):
        v = [1, 1, 1, 1]
        head = helper(v)
        result = Solution1().removeNodes(head)
        print_list(result)


    def test12(self):
        v = [5, 2, 13, 3, 8]
        head = helper(v)
        result = Solution2().removeNodes(head)
        print_list(result)

    def test22(self):
        v = [1, 1, 1, 1]
        head = helper(v)
        result = Solution2().removeNodes(head)
        print_list(result)

    def test22(self):
        v = [100, 101, 5, 10]
        head = helper(v)
        result = Solution2().removeNodes(head)
        print_list(result)

if __name__ == '__main__':
    unittest.main()