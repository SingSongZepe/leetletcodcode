
from typing import List

from itertools import combinations

class Solution:
    def largestTriangleArea(self, points: List[List[int]]) -> float:
        return max(
            # formula for area of triangle: 0.5 * |x1(y2 - y3) + x2(y3 - y1) + x3(y1 - y2)|
            0.50000 * abs(
                x1 * (y2 - y3) +
                x2 * (y3 - y1) +
                x3 * (y1 - y2)
            )
            for (x1, y1), (x2, y2), (x3, y3) in combinations(points, 3)
        )


class Solution1:
    def largestTriangleArea(self, points: List[List[int]]) -> float:
        def cross(v1, v2):
            return v1[0] * v2[1] - v2[0] * v1[1]

        def vector(p1, p2):
            return (p1[0] - p2[0], p1[1] - p2[1])

        def convexHull(points):
            # Sort points lexicographically
            points = sorted(points)
            # Build the lower and upper hulls of the convex hull
            lower, upper = [], []
            for point in points:
                while (
                    len(lower) >= 2
                    and cross(vector(lower[-2], lower[-1]), vector(lower[-1], point))
                    <= 0
                ):
                    lower.pop()
                while (
                    len(upper) >= 2
                    and cross(vector(upper[-2], upper[-1]), vector(upper[-1], point))
                    >= 0
                ):
                    upper.pop()
                lower.append(point)
                upper.append(point)
            return lower[:-1] + upper[::-1][:-1]

        polygon = convexHull(points)

        n = len(polygon)
        max_area = 0

        for i in range(n):
            k = (i + 2) % n
            for j in range(i + 1, n):
                while True:
                    current_area = abs(
                        cross(
                            vector(polygon[i], polygon[j]),
                            vector(polygon[i], polygon[k]),
                        )
                    )
                    next_k = (k + 1) % n
                    next_area = abs(
                        cross(
                            vector(polygon[i], polygon[j]),
                            vector(polygon[i], polygon[next_k]),
                        )
                    )
                    if next_area > current_area:
                        k = next_k
                    else:
                        break
                max_area = max(max_area, current_area)

        return max_area / 2


def main():
    print('Hello World')

if __name__ == '__main__':
    main()