

from typing import List

class Solution:
    def maximumSumOfHeights(self, heights: List[int]):
        n = len(heights)
        leftsum = [0] * n
        rightsum = [0] * n

        leftsmall = [-1] * n
        rightsmall = [n] * n

        leftsum[0] = heights[0]
        rightsum[n-1] = heights[n-1]

        st = [0]

        for i in range(1, n):
            while st and heights[st[-1]] >= heights[i]:
                st.pop()
            if st:
                leftsmall[i] = st[-1]
            lsmall = leftsmall[i]
            cnt = i - lsmall
            if lsmall > -1:
                leftsum[i] = leftsum[lsmall] + cnt * heights[i]
            else:
                leftsum[i] = cnt * heights[i]
            st.append(i)

        st = [n-1]


        for i in range(n-2, -1, -1):
            while st and heights[st[-1]] >= heights[i]:
                st.pop()
            if st:
                rightsmall[i] = st[-1]
            rsmall = rightsmall[i]
            cnt = rsmall - i
            if rsmall < n:
                rightsum[i] = rightsum[rsmall] + cnt * heights[i]
            else:
                rightsum[i] = cnt * heights[i]
            st.append(i)

        maxcount = 0
        for i in range(n):
            maxcount = max(maxcount, leftsum[i] + rightsum[i] - heights[i])

        return maxcount



class Solution3:
    def maximumSumOfHeights(self, maxHeights):
        n = len(maxHeights)
        prevsmall = [-1] * n
        nextsmall = [n] * n
        leftsum = [0] * n
        rightsum = [0] * n
        leftsum[0] = maxHeights[0]
        st = [0]

        # Calculate prevSmall
        for i in range(1, n):
            while st and maxHeights[st[-1]] >= maxHeights[i]:
                st.pop()
            if st:
                prevsmall[i] = st[-1]
            prev = prevsmall[i]
            count = i - prev
            leftsum[i] += count * maxHeights[i]
            if prev != -1:
                leftsum[i] += leftsum[prev]
            st.append(i)

        while st:
            st.pop()
        st.append(n - 1)

        # Calculate nextsmall;
        rightsum[n - 1] = maxHeights[n - 1]
        for i in range(n - 2, -1, -1):
            while st and maxHeights[st[-1]] >= maxHeights[i]:
                st.pop()
            if st:
                nextsmall[i] = st[-1]
            next_ = nextsmall[i]
            count = next_ - i
            rightsum[i] += count * maxHeights[i]
            if next_ != n:
                rightsum[i] += rightsum[next_]
            st.append(i)

        maxcount = 0
        for i in range(n):
            maxcount = max(maxcount, leftsum[i] + rightsum[i] - maxHeights[i])

        return maxcount

from functools import lru_cache
class Solution1:
    def maximumSumOfHeights(self, maxHeights: List[int]) -> int:
        n = len(maxHeights)

        @lru_cache(maxsize=4096)
        def right(prev, i):
            if n <= i:
                return prev
            return prev + right(min(prev, maxHeights[i]), i + 1)

        @lru_cache(maxsize=4096)
        def left(prev, i):
            if i < 0:
                return prev
            return prev + left(min(prev, maxHeights[i]), i - 1)

        answ = 0
        for peak in range(n):
            prev = maxHeights[peak]
            if prev * n < answ:
                continue
            answ = max(answ, left(prev, peak - 1) + right(prev, peak + 1) - prev)
        return answ

class Solution2:   # Brute Force

    def maximumSumOfHeights(self, maxHeights) -> int:
        n = len(maxHeights)
        maxSum = -float('inf')
        for peak in range(n):
            i, j = peak-1, peak+1
            mountain = [float('inf') for i in range(n)]
            mountain[peak] = maxHeights[peak]

            while i >= 0:
                mountain[i] = min(maxHeights[i], mountain[i+1])
                i -= 1
            while j < n:
                mountain[j] = min(maxHeights[j], mountain[j-1])
                j += 1

            maxSum = max(sum(mountain), maxSum)
        return maxSum





def main():
    print('Hello World')

if __name__ == '__main__':
    main()