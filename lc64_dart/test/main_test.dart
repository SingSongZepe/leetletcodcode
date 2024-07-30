import 'package:lc64_dart/main_lib.dart';
import 'package:test/test.dart';

void main() {
  test('test1', () {
    Solution solution = Solution();
    List<List<int>> nums = [
      [1,3,1],
      [1,5,1],
      [4,2,1]
    ];
    int result = solution.minPathSum(nums);
    print(result);
  });

  test('test2', () {
    Solution solution = Solution();
    List<List<int>> nums = [
      [1, 2, 3],
      [4, 5, 6],
    ];
    int result = solution.minPathSum(nums);
    print(result);
  });

    test('test11', () {
    Solution1 solution = Solution1();
    List<List<int>> nums = [
      [1,3,1],
      [1,5,1],
      [4,2,1]
    ];
    int result = solution.minPathSum(nums);
    print(result);
  });

  test('test21', () {
    Solution1 solution = Solution1();
    List<List<int>> nums = [
      [1, 2, 3],
      [4, 5, 6],
    ];
    int result = solution.minPathSum(nums);
    print(result);
  });
}
