
from typing import List


# 130 milliseconds
class Solution:
    def __init__(self):
        self.my_grid = []

    def floodfill(self, grid, r, c, color):
        # Check for out-of-bounds or already colored cells
        if r < 0 or r >= len(grid) or c < 0 or c >= len(grid[0]) or grid[r][c] != 0:
            return
        
        # Color the cell
        grid[r][c] = color
        
        # Flood fill in all four directions
        self.floodfill(grid, r - 1, c, color)
        self.floodfill(grid, r + 1, c, color)
        self.floodfill(grid, r, c - 1, color)
        self.floodfill(grid, r, c + 1, color)

    def regionsBySlashes(self, grid: List[str]) -> int:
        #variables
        n = len(grid)
        max_color = 0

        #convert strings to grid with -1,0
        self.my_grid = [[0 for i in range(n*3)] for j in range(n*3)]
        print(self.my_grid)
        for i in range(n):
            for j in range(n):
                if grid[i][j]=="/":
                    self.my_grid[i*3+0][j*3+2] = -1
                    self.my_grid[i*3+1][j*3+1] = -1
                    self.my_grid[i*3+2][j*3+0] = -1
                elif grid[i][j]=="\\":
                    self.my_grid[i*3+0][j*3+0] = -1
                    self.my_grid[i*3+1][j*3+1] = -1
                    self.my_grid[i*3+2][j*3+2] = -1
                    
        print(self.my_grid)

        #apply flood fill with m as color till entire grid is not colored
        for i in range(n*3):
            for j in range(n*3):
                if self.my_grid[i][j]==0:
                    max_color += 1
                    self.floodfill(self.my_grid, i, j, max_color)
                

        return max_color        
        
# 40 milliseconds
class Solution:
    def regionsBySlashes(self, grid: List[str]) -> int:
        grid_size = len(grid)
        points_per_side = grid_size + 1
        total_points = points_per_side * points_per_side

        # Initialize disjoint set data structure
        parent_array = [-1] * total_points

        # Connect border points
        for i in range(points_per_side):
            for j in range(points_per_side):
                if (
                    i == 0
                    or j == 0
                    or i == points_per_side - 1
                    or j == points_per_side - 1
                ):
                    point = i * points_per_side + j
                    parent_array[point] = 0

        # Set the parent of the top-left corner to itself
        parent_array[0] = -1
        region_count = 1  # Start with one region (the border)

        # Process each cell in the grid
        for i in range(grid_size):
            for j in range(grid_size):
                # Treat each slash as an edge connecting two points
                if grid[i][j] == "/":
                    top_right = i * points_per_side + (j + 1)
                    bottom_left = (i + 1) * points_per_side + j
                    region_count += self._union(
                        parent_array, top_right, bottom_left
                    )
                elif grid[i][j] == "\\":
                    top_left = i * points_per_side + j
                    bottom_right = (i + 1) * points_per_side + (j + 1)
                    region_count += self._union(
                        parent_array, top_left, bottom_right
                    )

        return region_count

    def _find(self, parent_array: List[int], node: int) -> int:
        if parent_array[node] == -1:
            return node

        parent_array[node] = self._find(parent_array, parent_array[node])
        return parent_array[node]

    def _union(self, parent_array: List[int], node1: int, node2: int) -> int:
        parent1 = self._find(parent_array, node1)
        parent2 = self._find(parent_array, node2)

        if parent1 == parent2:
            return 1  # Nodes are already in the same set, new region formed

        parent_array[parent2] = parent1  # Union the sets
        return 0  # No new region formed


class Solution:
    def regionsBySlashes(self, grid: List[str]) -> int:


        pass




def main():
    print('Hello World')

if __name__ == '__main__':
    main()