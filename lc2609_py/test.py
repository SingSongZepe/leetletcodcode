import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        s = '01000111'
        result = Solution().findTheLongestBalancedSubstring(s)
        print(result)

    def test2(self):
        s = '00111'
        result = Solution().findTheLongestBalancedSubstring(s)
        print(result)

    def test3(self):
        s = '111'
        result = Solution().findTheLongestBalancedSubstring(s)
        print(result)

    # Solution 1

    def test11(self):
        s = '01000111'
        result = Solution1().findTheLongestBalancedSubstring(s)
        print(result)

    def test21(self):
        s = '00111'
        result = Solution1().findTheLongestBalancedSubstring(s)
        print(result)

    def test31(self):
        s = '111'
        result = Solution1().findTheLongestBalancedSubstring(s)
        print(result)



if __name__ == '__main__':
    unittest.main()