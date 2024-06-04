import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, n, leftChild, rightChild, expected=None):
        result = Solution().validateBinaryTreeNodes(n, leftChild, rightChild)
        print(result)

    def t1(self, n, leftChild, rightChild, expected=None):
        result = Solution1().validateBinaryTreeNodes(n, leftChild, rightChild)
        print(result)

    def t2(self, n, leftChild, rightChild, expected=None):
        result = Solution2().validateBinaryTreeNodes(n, leftChild, rightChild)
        print(result)

    def test1(self):
        n = 4
        leftChild = [1,-1,3,-1]
        rightChild = [2,-1,-1,-1]
        self.t(n, leftChild, rightChild) # expected: True


    def test2(self):
        n = 4
        leftChild = [1, -1, 3, -1]
        rightChild = [2, 3, -1, -1]
        self.t(n, leftChild, rightChild) # expected: False

    def test3(self):
        n = 2
        leftChild = [1, 0]
        rightChild = [-1, -1]
        self.t(n, leftChild, rightChild) # expected: False

    def test4(self):
        n = 6
        leftChild = [1, -1, -1, 4, -1, -1]
        rightChild = [2, -1, -1, 5, -1, -1]
        self.t(n, leftChild, rightChild) # expected: False

    def test5(self):
        n = 4
        leftChild = [3,-1,1,-1]
        rightChild = [-1,-1,0,-1]
        self.t(n, leftChild, rightChild) # expected: True

    def test21(self):
        n = 4
        leftChild = [1, -1, 3, -1]
        rightChild = [2, 3, -1, -1]
        self.t1(n, leftChild, rightChild) # expected: False

    def test31(self):
        n = 2
        leftChild = [1, 0]
        rightChild = [-1, -1]
        self.t1(n, leftChild, rightChild) # expected: False

    def test41(self):
        n = 6
        leftChild = [1, -1, -1, 4, -1, -1]
        rightChild = [2, -1, -1, 5, -1, -1]
        self.t1(n, leftChild, rightChild) # expected: False

    def test51(self):
        n = 4
        leftChild = [3,-1,1,-1]
        rightChild = [-1,-1,0,-1]
        self.t1(n, leftChild, rightChild) # expected: True

    def test61(self):
        n = 4
        leftChild = [1,0,3,-1]
        rightChild = [-1,-1,-1,-1]
        self.t1(n, leftChild, rightChild) # expected: False

    def test71(self):
        n = 4
        leftChild = [1,2,0,-1]
        rightChild = [-1,-1,-1,-1]
        self.t1(n, leftChild, rightChild) # expected: False

    def test62(self):
        n = 4
        leftChild = [1,0,3,-1]
        rightChild = [-1,-1,-1,-1]
        self.t2(n, leftChild, rightChild) # expected: False

    def test72(self):
        n = 4
        leftChild = [1,2,0,-1]
        rightChild = [-1,-1,-1,-1]
        self.t2(n, leftChild, rightChild) # expected: False


if __name__ == '__main__':
    unittest.main()