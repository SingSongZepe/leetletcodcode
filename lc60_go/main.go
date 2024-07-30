package main

import (
	"fmt"
	"strconv"
	"strings"
)

func getLet(n int, k int) int {
	fac := 1
	let := 1
	for i := 2; i <= n; i++ {
		fac *= i
		if k <= fac {
			let = n - i + 1
			break
		}
	}
	return let
}

func getPermutation(n int, k int) string {
	let := getLet(n, k)
	var count *int = new(int)
	var result *string = new(string)
	var nums []int
	var used []bool
	var path []int
	for i := let; i <= n; i++ {
		nums = append(nums, i)
	}
	for i := 0; i < n; i++ {
		used = append(used, false)
	}
	generatePermutations(nums, used, path, count, k, result)
	var prefix string
	for i := 1; i < let; i++ {
		prefix += strconv.Itoa(i)
	}
	return prefix + *result
}

func generatePermutations(nums []int, used []bool, path []int, count *int, k int, result *string) {
	if len(path) == len(nums) {
		*count++
		if *count == k && *result == "" {
			var strNums []string
			for _, num := range path {
				strNums = append(strNums, strconv.Itoa(num))
			}
			*result = strings.Join(strNums, "")
			return
		}
		return
	} else {
		for i := 0; i < len(nums); i++ {
			if !used[i] {
				path = append(path, nums[i])
				used[i] = true
				generatePermutations(nums, used, path, count, k, result)
				path = path[:len(path)-1]
				used[i] = false
			}
		}
	}
}

func getPermutation1(n int, k int) string {
	numbers := make([]int, n)
	for i := 0; i < n; i++ {
		numbers[i] = i + 1
	}
	factorials := make([]int, n+1)
	factorials[0] = 1
	fac := 1
	for i := 1; i <= n; i++ {
		factorials[i] = fac
		fac *= (i + 1)
	}
	result := ""
	k--
	for n > 0 {
		index := k / factorials[n-1]
		result += strconv.Itoa(numbers[index])
		numbers = append(numbers[:index], numbers[index+1:]...)
		k %= factorials[n-1]
		n--
	}
	return result
}

var factorials = []int{
	0,
	1,
	2,
	6,
	24,
	120,
	720,
	5040,
	40320,
	362880,
	3628800,
}

func getPermutation2(n int, k int) string {
	sb := strings.Builder{}
	set := make([]int, n)
	for i := 0; i < n; i++ {
		set[i] = i + 1
	}

	// j is 0 based k
	j := k - 1
	for i := n - 1; i >= 0; i-- {
		if i == 0 {
			sb.WriteString(fmt.Sprintf("%d", set[0]))
			break
		}

		// iv = j / (n-1)!
		iv := j / factorials[i]

		// j1 = j0 % (n-1)!
		j = j % factorials[i]

		// Write the value at idx iv and remove it from the set
		v := set[iv]
		set = append(set[:iv], set[iv+1:]...)
		sb.WriteString(fmt.Sprintf("%d", v))
	}

	return sb.String()
}

// func factorial(n int) int {
// 	result := 1
// 	for i := 2; i <= n; i++ {
// 		result *= i
// 	}
// 	return result
// }

func main() {
	fmt.Println("Hello, world!")
}
