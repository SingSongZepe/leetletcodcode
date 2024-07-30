import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, operations, expected=None):
        result = Solution().finalValueAfterOperations(operations)
        print(result)

    def test1(self):
        operations = ["--X","X++","X++"]
        self.t(operations)

    def test2(self):
        operations = ["++X","++X","X++"]
        self.t(operations)

    def test3(self):
        operations = ["X++","++X","--X","X--"]
        self.t(operations)



if __name__ == '__main__':
    unittest.main()