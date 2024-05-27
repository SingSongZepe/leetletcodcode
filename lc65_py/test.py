import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        s = '0'
        result = Solution().isNumber(s)
        print(result) # expected output: True

    def test2(self):
        s = 'e'
        result = Solution().isNumber(s)
        print(result) # expected output: False

    def test3(self):
        s = '.'
        result = Solution().isNumber(s)
        print(result) # expected output: False

    def test4(self):
        s = '-0.1'
        result = Solution().isNumber(s)
        print(result) # expected output: False

    def test5(self):
        s = ".0e7"
        result = Solution().isNumber(s)
        print(result)  # expected output: False

    def test6(self):
        s = "46.e3"
        result = Solution().isNumber(s)
        print(result)  # expected output: False

    def test_state(self):
        s = State()
        s.change_mode('1')
        print(s.mode) # expected output: 3



    def test_all_true(self):
        ss = ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]
        for s in ss:
            result = Solution().isNumber(s)
            print(result)

    def test_all_false(self):
        ss = ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]
        for s in ss:
            result = Solution().isNumber(s)
            print(result)

if __name__ == '__main__':
    unittest.main()