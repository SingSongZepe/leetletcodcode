

class Solution:
    def equalSubstring(self, s: str, t: str, maxCost: int) -> int:

        diff = []
        for i in range(len(s)):
            diff.append(abs(ord(s[i]) - ord(t[i])))
        del s, t

        front = 0
        ml = 0
        curr_cost = 0
        for rear in range(len(diff)):
            curr_cost += diff[rear]

            while curr_cost > maxCost:
                curr_cost -= diff[front]
                front += 1

            ml = max(ml, rear - front + 1)

        return ml

class Solution1:
    def equalSubstring(self, s: str, t: str, maxCost: int) -> int:
        l = len(s)
        front = 0

        curr_cost = 0
        ml = 0
        for rear in range(l):
            curr_cost += abs(ord(s[rear]) - ord(t[rear]))
            while curr_cost > maxCost:
                curr_cost -= abs(ord(s[front]) - ord(t[front]))
                front += 1
            ml = max(ml, rear - front + 1)
        return ml



class Solution2:
    def equalSubstring(self, s: str, t: str, maxCost: int) -> int:
        n = len(s)
        start = 0
        current_cost = 0
        max_length = 0

        for end in range(n):
            current_cost += abs(ord(s[end]) - ord(t[end]))

            while current_cost > maxCost:
                current_cost -= abs(ord(s[start]) - ord(t[start]))
                start += 1

            max_length = max(max_length, end - start + 1)

        return max_length


def main():
    print('Hello World')

if __name__ == '__main__':
    main()