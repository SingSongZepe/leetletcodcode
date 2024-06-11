import unittest
from main import *


class Test(unittest.TestCase):
    def t(self, arg, times, expected=None):
        result = Solution().alertNames(arg, times)
        print(result)
        self.assertEqual(result, expected)

    def t1(self, arg, times, expected=None):
        result = Solution1().alertNames(arg, times)
        print(result)
        self.assertEqual(result, expected)

    def t2(self, arg, times, expected=None):
        result = Solution1().alertNames(arg, times)
        print(result)
        self.assertEqual(result, expected)
        
    def test1(self):
        keyName = ["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"]
        keyTime = ["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"]
        self.t(keyName, keyTime, ["daniel"])

    def test2(self):
        keyName = ["alice", "alice", "alice", "bob", "bob", "bob", "bob"]
        keyTime = ["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"]
        self.t(keyName, keyTime, ["bob"])

    def test3(self):
        keyName = ["leslie","leslie","leslie","clare","clare","clare","clare"]
        keyTime = ["13:00","13:20","14:00","18:00","18:51","19:30","19:49"]
        self.t(keyName, keyTime, ["clare", "leslie"])

    def test4(self):
        keyName = ["john","john","john"]
        keyTime = ["23:58","23:59","00:01"]
        self.t(keyName, keyTime, [])

    def test5(self):
        keyName = ["a","a","a","a","a","b","b","b","b","b","b"]
        keyTime = ["04:48","23:53","06:36","07:45","12:16","00:52","10:59","17:16","00:36","01:26","22:42"]
        self.t(keyName, keyTime, ['b'])


    def test11(self):
        keyName = ["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"]
        keyTime = ["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"]
        self.t(keyName, keyTime, ["daniel"])

    def test21(self):
        keyName = ["alice", "alice", "alice", "bob", "bob", "bob", "bob"]
        keyTime = ["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"]
        self.t1(keyName, keyTime, ["bob"])

    def test31(self):
        keyName = ["leslie","leslie","leslie","clare","clare","clare","clare"]
        keyTime = ["13:00","13:20","14:00","18:00","18:51","19:30","19:49"]
        self.t1(keyName, keyTime, ["clare", "leslie"])

    def test41(self):
        keyName = ["john","john","john"]
        keyTime = ["23:58","23:59","00:01"]
        self.t1(keyName, keyTime, [])

    def test51(self):
        keyName = ["a","a","a","a","a","b","b","b","b","b","b"]
        keyTime = ["04:48","23:53","06:36","07:45","12:16","00:52","10:59","17:16","00:36","01:26","22:42"]
        self.t1(keyName, keyTime, ['b'])

    def test32(self):
        keyName = ["leslie","leslie","leslie","clare","clare","clare","clare"]
        keyTime = ["13:00","13:20","14:00","18:00","18:51","19:30","19:49"]
        self.t2(keyName, keyTime, ["clare", "leslie"])

    def test42(self):
        keyName = ["john","john","john"]
        keyTime = ["23:58","23:59","00:01"]
        self.t2(keyName, keyTime, [])

    def test52(self):
        keyName = ["a","a","a","a","a","b","b","b","b","b","b"]
        keyTime = ["04:48","23:53","06:36","07:45","12:16","00:52","10:59","17:16","00:36","01:26","22:42"]
        self.t2(keyName, keyTime, ['b'])

    def test_in1hour(self):
        from datetime import datetime
        def in1hour(t1, t2): # t2 > t1
            format = "%H:%M"
            time_format = lambda t: datetime.strptime(t, format)
            t1 = time_format(t1)
            t2 = time_format(t2)
            diff = (t2 - t1).total_seconds()
            return diff <= 3600

        t1 = "10:00"
        t2 = "10:30"
        print(in1hour(t1, t2))

        t1 = "10:00"
        t2 = "11:01"
        print(in1hour(t1, t2))

if __name__ == '__main__':
    unittest.main()