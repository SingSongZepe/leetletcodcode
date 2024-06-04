import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, date1, date2, expected=None):
        result = Solution().daysBetweenDates(date1, date2)
        print(result)

    def t1(self, date1, date2, expected=None):
        result = Solution1().daysBetweenDates(date1, date2)
        print(result)

    def test1(self):
        date1 = '2019-06-29'
        date2 = '2019-06-30'
        self.t(date1, date2)

    def test2(self):
        date1 = '2020-01-15'
        date2 = '2019-12-31'
        self.t(date1, date2)

    def test3(self):
        date1 = "2009-08-18"
        date2 = "2080-08-08"
        self.t(date1, date2)

    def test11(self):
        date1 = '2019-06-29'
        date2 = '2019-06-30'
        self.t1(date1, date2)

    def test21(self):
        date1 = '2020-01-15'
        date2 = '2019-12-31'
        self.t1(date1, date2)

    def test31(self):
        date1 = "2009-08-18"
        date2 = "2080-08-08"
        self.t1(date1, date2)

    def test_days(self):
        date1 = '1971-1-2'
        date2 = '1972-1-1'
        date1 = date1.split('-')
        date2 = date2.split('-')
        y1 = int(date1[0])
        y2 = int(date2[0])
        m1 = int(date1[1])
        m2 = int(date2[1])
        d1 = int(date1[2])
        d2 = int(date2[2])

        d = days(y1, m1, d1)
        print(d)
        d = days(y2, m2, d2)
        print(d)

if __name__ == '__main__':
    unittest.main()