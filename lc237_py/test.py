import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        v = [4, 5, 1, 9]
        n = 5
        head = helper(v)
        node = get_node_by_val(head, n)
        Solution().deleteNode(node)

        print_list(head)

    def test2(self):
        v = [4, 5, 1, 9]
        n = 1
        head = helper(v)
        node = get_node_by_val(head, n)
        Solution().deleteNode(node)

        print_list(head)



if __name__ == '__main__':
    unittest.main()