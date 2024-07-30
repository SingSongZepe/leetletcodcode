import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        s = 'leetcode'
        result = Solution().firstUniqChar(s)
        print(result) # expected output: 0

    def test2(self):
        s = 'loveleetcode'
        result = Solution().firstUniqChar(s)
        print(result) # expected output: 2

    def test3(self):
        s = 'aabb'
        result = Solution().firstUniqChar(s)
        print(result) # expected output: -1

    def test4(self):
        s = 'lloooooveleetcode'
        result = Solution().firstUniqChar(s)
        print(result) # expected output: 2


    def test11(self):
        s = 'leetcode'
        result = Solution1().firstUniqChar(s)
        print(result) # expected output: 0

    def test21(self):
        s = 'loveleetcode'
        result = Solution1().firstUniqChar(s)
        print(result) # expected output: 2

    def test31(self):
        s = 'aabb'
        result = Solution1().firstUniqChar(s)
        print(result) # expected output: -1

    def test41(self):
        s = 'lloooooveleetcode'
        result = Solution1().firstUniqChar(s)
        print(result) # expected output: 2

if __name__ == '__main__':
    unittest.main()