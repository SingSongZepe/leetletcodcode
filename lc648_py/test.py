import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, sentence, expected=None):
        result = Solution().replaceWords(arg, sentence)
        print(result)
        
    def test1(self):
        dictionary = ["cat", "bat", "rat"]
        sentence = "the cattle was rattled by the battery"
        self.t(dictionary, sentence)

    def test2(self):
        dictionary = ["a", "b", "c"]
        sentence = "aadsfasf absbs bbab cadsfafs"
        self.t(dictionary, sentence)

    def test3(self):
        dictionary = ["catt", "cat", "bat", "rat", "ttttt"]
        sentence = "the cattle was rattled by the battery"
        self.t(dictionary, sentence)



    def test_while(self):

        i = 0
        l = [1 for _ in range(100)]
        count = 0
        while i < len(l):
            l.pop()
            count += 1
            i += 1

        print(count)

if __name__ == '__main__':
    unittest.main()