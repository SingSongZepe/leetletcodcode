#include <iostream>
#include <utility>
#include <vector>
// #include <unordered_map>
// #include "gtest/gtest.h"


using std::vector;
class Solution {
public:
    int minPathSum(vector<vector<int>>& grid) {
        // std::unordered_map<std::pair<int, int>, int> memo; 
        std::vector<std::vector<int>> memo(grid.size(), std::vector<int>(grid[0].size(), 0));
        return move(grid, 0, 0, memo);
    }
    int move(vector<vector<int>>& grid, int i, int j, std::vector<std::vector<int>>& memo) {
        if (i == grid.size()-1 && j == grid[0].size()-1) {
            return grid[i][j];
        };
        if (memo[i][j]) {
            return memo[i][j];
        }
        int right = move(grid, i, j + 1, memo);
        int down = move(grid, i + 1, j, memo);
        int min_sum = std::min(right, down) + grid[i][j];
        memo[i][j] = min_sum;
        return min_sum;
    }
};

int main() {
    Solution s;
    vector<vector<int>> grid = 
    {{1,3,1},
    {1,5,1},
    {4,2,1}};
    std::cout << s.minPathSum(grid) << std::endl;
}
