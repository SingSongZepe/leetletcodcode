import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        head = helper([[7,None],[13,0],[11,4],[10,2],[1,0]])
        print_list(head)
        result = Solution().copyRandomList(head)
        print_list(result)

    def test2(self):
        head = helper([[7,None],[13,0],[11,4],[10,2],[1,0]])
        print_list(head)
        result = Solution().copyRandomList(head)
        print_list(result)

if __name__ == '__main__':
    unittest.main()