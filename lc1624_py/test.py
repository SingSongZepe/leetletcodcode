import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        s = 'aa'
        result = Solution().maxLengthBetweenEqualCharacters(s)
        print(result)

    def test2(self):
        s = 'abca'
        result = Solution().maxLengthBetweenEqualCharacters(s)
        print(result)

    def test3(self):
        s = 'cbzxy'
        result = Solution().maxLengthBetweenEqualCharacters(s)
        print(result)

    def test4(self):
        s = 'cabbac'
        result = Solution().maxLengthBetweenEqualCharacters(s)
        print(result)

    def test5(self):
        s = 'abcdefghijklmnopqrstuvwxyz'
        result = Solution().maxLengthBetweenEqualCharacters(s)
        print(result)

    def test6(self):
        s = 'aaabbbccc'
        result = Solution().maxLengthBetweenEqualCharacters(s)
        print(result)

    def test7(self):
        s = 'abcabcbb'
        result = Solution().maxLengthBetweenEqualCharacters(s)
        print(result)

    def test8(self):
        s = 'bbbbb'
        result = Solution().maxLengthBetweenEqualCharacters(s)
        print(result)

if __name__ == '__main__':
    unittest.main()