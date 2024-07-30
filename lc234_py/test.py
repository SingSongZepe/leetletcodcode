import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, expected=None):
        result = Solution().isPalindrome(arg)
        print(result)

    def t1(self, arg, expected=None):
        result = Solution1().isPalindrome(arg)
        print(result)
        
    def test1(self):
        v = [1, 2, 2, 1]
        head = helper(v)
        self.t(head, True)

    def test2(self):
        v = [1, 2, 3]
        head = helper(v)
        self.t(head, False)

    def test3(self):
        v = [1, 1]
        head = helper(v)
        self.t(head, True)

    def test4(self):
        v = [1]
        head = helper(v)
        self.t(head, True)

    def test5(self):
        v = []
        head = helper(v)
        self.t(head, True)

    def test11(self):
        v = [1, 2, 2, 1]
        head = helper(v)
        self.t1(head, True)

    def test21(self):
        v = [1, 2, 3]
        head = helper(v)
        self.t1(head, False)

    def test31(self):
        v = [1, 1]
        head = helper(v)
        self.t1(head, True)

    def test41(self):
        v = [1]
        head = helper(v)
        self.t1(head, True)

    def test51(self):
        v = []
        head = helper(v)
        self.t1(head, True)

if __name__ == '__main__':
    unittest.main()