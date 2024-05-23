import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        word = "abcdefd"
        ch = "d"
        result = Solution().reversePrefix(word, ch)
        print(result)

    def test2(self):
        word = "xyxzxe"
        ch = "z"
        result = Solution().reversePrefix(word, ch)
        print(result)

    def test3(self):
        word = "abcd"
        ch = "z"
        result = Solution().reversePrefix(word, ch)
        print(result)

    def test4(self):
        word = "abcdz"
        ch = "z"
        result = Solution().reversePrefix(word, ch)
        print(result)

    ###
    def test11(self):
        word = "abcdefd"
        ch = "d"
        result = Solution1().reversePrefix(word, ch)
        print(result)

    def test21(self):
        word = "xyxzxe"
        ch = "z"
        result = Solution1().reversePrefix(word, ch)
        print(result)

    def test31(self):
        word = "abcd"
        ch = "z"
        result = Solution1().reversePrefix(word, ch)
        print(result)

    def test41(self):
        word = "abcdz"
        ch = "z"
        result = Solution1().reversePrefix(word, ch)
        print(result)


    def test32(self):
        word = "abcd"
        ch = "z"
        result = Solution2().reversePrefix(word, ch)
        print(result)

    def test42(self):
        word = "abcdz"
        ch = "z"
        result = Solution2().reversePrefix(word, ch)
        print(result)

    def test_index(self):
        word = "abcdb"
        print(word.index('z'))
        print(word[:(word.index('b')+1)][::-1]+word[(word.index('b')+1):])

    def test_find(self):
        word = "abcdb"
        print(word.find('z'))
        # print(word[:(word.index('b')+1)][::-1]+word[(word.index('b')+1):])

if __name__ == '__main__':
    unittest.main()