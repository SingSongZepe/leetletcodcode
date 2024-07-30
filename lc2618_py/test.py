import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        v = [1, 8, 9]
        head = helper(v)
        result = Solution().doubleIt(head)
        print_list(result) # expected output: [3, 7, 8]

    def test2(self):
        v = [9, 9, 9]
        head = helper(v)
        result = Solution().doubleIt(head)
        print_list(result) # expected output: [1, 9, 9, 8]

if __name__ == '__main__':
    unittest.main()