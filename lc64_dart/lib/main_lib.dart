
// import 'dart:ffi';

int add(int a, int b) {
  return a + b;
}

class Solution {
  int minPathSum(List<List<int>> grid) {
    List<List<int>> minmap = List.generate(grid.length, (i) => List.generate(grid[0].length, (j) => 0));  
    return minPath(grid, 0, 0, grid.length, grid[0].length, minmap);
  }
  int minPath(List<List<int>> grid, int r, int c, int m, int n, List<List<int>> minmap) {
    if (r == m - 1 && c == n - 1) {
      return grid[r][c];
    }
    if (r >= m || c >= n) {
      return 2147483647;
    }
    if (minmap[r][c]!= 0) {
      return minmap[r][c];
    }

    // if
    int right = minPath(grid, r, c + 1, m, n, minmap);
    int down = minPath(grid, r + 1, c, m, n, minmap);
    int MIN = min(down, right) + grid[r][c];
    minmap[r][c] = MIN;
    return MIN;
  }
  int min(int a, int b) { 
    if (a < b) {
      return a;
    }
    return b;
  }
}

class Solution1 {
  int minPathSum(List<List<int>> grid) {
      int m = grid.length;
      int n = grid[0].length;

      List<int> pathSum = List.filled(n, 100000);
      pathSum[0] = 0;

      for (int y=0; y<m; y++) {
        for (int x=0; x<n; x++) {
          pathSum[x] = (x > 0 ? min(pathSum[x], pathSum[x-1]) : pathSum[x]) + grid[y][x];
        }
      }
      return pathSum[n-1];
  }
  int min(int a, int b) {
    if (a<b){
      return a;
    }
    return b;
  }
}
