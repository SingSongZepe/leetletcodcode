import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, books, shelfWidth, expected=None):
        result = Solution().minHeightShelves(books, shelfWidth)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        books = [[1,1],[2,3],[2,3],[1,1],[1,1],[1,1],[1,2]]
        shelfWidth = 4
        expected = 6
        self.t(books, shelfWidth, expected)
    
    def test2(self):
        books = [[1,3],[2,4],[3,2]]
        shelfWidth = 6
        expected = 4
        self.t(books, shelfWidth, expected)


if __name__ == '__main__':
    unittest.main()