import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        words = ["dog", "cat", "dad", "good"]
        letters = ["a", "a", "c", "d", "d", "d", "g", "o", "o"]
        score = [1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        result = Solution().maxScoreWords(words, letters, score)
        print(result)

    def test2(self):
        words = ["xxxz","ax","bx","cx"]
        letters = ["z","a","b","c","x","x","x"]
        score = [4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10]
        result = Solution().maxScoreWords(words, letters, score)
        print(result)

    def test3(self):
        words = ["leetcode"]
        letters = ["l", "e", "t", "c", "o", "d"]
        score = [0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
        result = Solution().maxScoreWords(words, letters, score)
        print(result)


    def test11(self):
        words = ["dog", "cat", "dad", "good"]
        letters = ["a", "a", "c", "d", "d", "d", "g", "o", "o"]
        score = [1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        result = Solution1().maxScoreWords(words, letters, score)
        print(result)

    def test21(self):
        words = ["xxxz","ax","bx","cx"]
        letters = ["z","a","b","c","x","x","x"]
        score = [4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10]
        result = Solution1().maxScoreWords(words, letters, score)
        print(result)

    def test31(self):
        words = ["leetcode"]
        letters = ["l", "e", "t", "c", "o", "d"]
        score = [0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
        result = Solution1().maxScoreWords(words, letters, score)
        print(result)
        
    def test_counter(self):
        words = [ "ax", "bx", "cx"]
        str = ''.join(words)
        x = Counter(str)
        y = Counter(["z", "a", "b", "c", "x", "x", "x"])
        print(x)
        print(y)
        print(x <= y)


        wordsz = ["xxz", "abx", "bx", "cx"]
        strz = ''.join(wordsz)
        z = Counter(strz)
        print(z)
        # print(x <= z)

if __name__ == '__main__':
    unittest.main()