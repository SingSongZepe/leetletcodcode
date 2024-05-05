int calculate() {
  return 6 * 7;
}

class Solution {
  int uniquePathsWithObstacles(List<List<int>> obstacleGrid) {
    Map<(int, int), int> memo = {}; 
    var m = obstacleGrid.length;
    var n = obstacleGrid[0].length;
    if (obstacleGrid[m-1][n-1] == 1 || obstacleGrid[0][0] == 1) {
      return 0;
    }
    return uniquePaths(obstacleGrid, 0, 0, m, n, memo);   
  }
  int uniquePaths(List<List<int>> obstacleGrid, int r, int c, int m, int n, Map<(int, int), int> memo) {
    if (r == m-1 && c == n-1) {
      return 1;
    }
    if (memo.containsKey((r,c))) {
      return memo[(r,c)] ?? 0;
    }
    int total = 0;
    if (c+1 != n && obstacleGrid[r][c+1] != 1) {
      total += uniquePaths(obstacleGrid, r, c+1, m, n, memo);
    }
    if (r+1 != m && obstacleGrid[r+1][c] != 1) {
      total += uniquePaths(obstacleGrid, r+1, c, m, n, memo);
    }
    memo[(r,c)] = total;
    return total;
  }
}


class Solution1 {
  int uniquePathsWithObstacles(List<List<int>> obstacleGrid) {
      Map<String,int> memo = {};
      return uniquePaths(obstacleGrid, [0,0], memo);
  }
  int uniquePaths(List<List<int>> grid , List<int> current_position, Map<String,int> memo){
      if(current_position[0] >= grid.length || current_position[1] >= grid[0].length || grid[current_position[0]][current_position[1]] == 1){
          return 0;
      }
      if(current_position[0] == grid.length - 1 && current_position[1] == grid[0].length - 1){
          return 1;
      }
      String current = current_position[0].toString() + ' ' + current_position[1].toString();

      if(memo.containsKey(current)){
          return(memo[current] ?? 0);
      }
      int return_paths = 0;
      return_paths += uniquePaths(grid, [current_position[0] + 1 , current_position[1]], memo);
      return_paths += uniquePaths(grid, [current_position[0] , current_position[1] + 1], memo);
      memo[current] = return_paths;
      return return_paths;
  }
}

class Solution2 {
  int uniquePathsWithObstacles(List<List<int>> obstacleGrid) {
      Map<(int, int),int> memo = {};
      return uniquePaths(obstacleGrid, [0,0], memo);
  }
  int uniquePaths(List<List<int>> grid , List<int> current_position, Map<(int, int),int> memo){
      if(current_position[0] >= grid.length || current_position[1] >= grid[0].length || grid[current_position[0]][current_position[1]] == 1){
          return 0;
      }
      if(current_position[0] == grid.length - 1 && current_position[1] == grid[0].length - 1){
          return 1;
      }

      if(memo.containsKey((current_position[0],current_position[1]))){
          return(memo[(current_position[0],current_position[1])] ?? 0);
      }
      int return_paths = 0;
      return_paths += uniquePaths(grid, [current_position[0] + 1 , current_position[1]], memo);
      return_paths += uniquePaths(grid, [current_position[0] , current_position[1] + 1], memo);
      memo[(current_position[0],current_position[1])] = return_paths;
      return return_paths;
  }
}