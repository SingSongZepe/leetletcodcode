137. Single Number II
Medium
Topics
Companies
Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.

You must implement a solution with a linear runtime complexity and use only constant extra space.

 

Example 1:

Input: nums = [2,2,3,2]
Output: 3
Example 2:

Input: nums = [0,1,0,1,0,1,99]
Output: 99
 

Constraints:

1 <= nums.length <= 3 * 104
-231 <= nums[i] <= 231 - 1
Each element in nums appears exactly three times except for one element which appears once.



### Learnt module: collections

1. `namedtuple`:
```python
from collections import namedtuple

Point = namedtuple('Point', ['x', 'y'])
p = Point(1, 2)
print(p.x, p.y)
```

2. `Counter`:
```python
from collections import Counter

data = ['apple', 'banana', 'apple', 'banana', 'cherry', 'apple']
counter = Counter(data)
print(counter)
```

3. `defaultdict`:
```python
from collections import defaultdict

default_dict = defaultdict(int)
default_dict['a'] = 1
print(default_dict['b'])  # 输出 0，因为默认类型是int，未定义的键值返回0
```

4. `deque`:
```python
from collections import deque

d = deque([1, 2, 3])
d.appendleft(0)
print(d)
```

5. `OrderedDict`:
```python
from collections import OrderedDict

ordered_dict = OrderedDict()
ordered_dict['a'] = 1
ordered_dict['b'] = 2
for key in ordered_dict:
    print(key, ordered_dict[key])
```

6. `ChainMap`:
```python
from collections import ChainMap

dict1 = {'a': 1, 'b': 2}
dict2 = {'b': 3, 'c': 4}
chain = ChainMap(dict1, dict2)
print(chain['a'])  # 输出 1
print(chain['b'])  # 输出 2，因为在 dict1 中查找到了 b 的值
print(chain['c'])  # 输出 4
```

### Solution 1

这个算法利用位运算来实现对出现一次的元素的计数，以实现线性时间复杂度和只使用常数额外空间的要求。下面详细解释为什么这个算法可以实现要求：

1. ones 和 twos 的意义：
   - `ones`表示当前位上数字出现1次的情况。
   - `twos`表示当前位上数字出现2次的情况。

2. 位运算的意义：
   - 对于当前位，若该位上的数字出现1次，则对应`ones`为1，`twos`为0。
   - 若该位上的数字出现2次，则对应`ones`为0，`twos`为1。

3. 更新规则：
   - 遍历数组中的每个数字，根据当前数字对`ones`和`twos`进行更新。
   - 对于`ones`，使用异或运算 (`^`) 可以实现当前位数字出现次数为1或3时的切换，然后利用与非运算 (`~`) 来控制只保留出现1次的情况，抹去出现3次的情况。
   - 对于`twos`，同样使用异或运算实现当前位数字出现次数为2或3时的切换，然后也利用与非运算来控制只保留出现2次的情况，抹去出现3次的情况。

4. 返回结果：
   - 最终返回的结果为`ones`，因为出现1次的元素只会影响到`ones`，而不会影响到`twos`。

因此，通过适当地更新`ones`和`twos`，只保留出现一次的元素影响的位，最终可以找出唯一出现一次的元素。这个算法能够满足题目对于线性时间复杂度和只使用常数额外空间的要求。
