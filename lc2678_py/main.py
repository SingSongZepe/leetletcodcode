

from typing import List
class Solution:
    def countSeniors(self, details: List[str]) -> int:
        return sum(1 for detail in details if int(detail[11:13]) > 60)



def main():
    print('Hello World')

if __name__ == '__main__':
    main()