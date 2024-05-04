package main

import "fmt"

func generateMatrix(n int) [][]int {
	result := make([][]int, n)
	for i := range result {
		result[i] = make([]int, n)
	}
	generate(0, result, n, 0)
	return result
}

func generate(from int, result [][]int, n int, startWith int) {
	if from >= (n+1)/2 {
		return
	}
	// top
	for i := from; i < n-from; i++ {
		startWith++
		result[from][i] = startWith
	}
	if n%2 == 1 && from == (n-1)/2 {
		return
	}
	// right
	for i := from + 1; i < n-from; i++ {
		startWith++
		result[i][n-from-1] = startWith
	}
	// bottom
	for i := n - from - 2; i >= from; i-- {
		startWith++
		result[n-from-1][i] = startWith
	}
	// left
	for i := n - from - 2; i > from; i-- {
		startWith++
		result[i][from] = startWith
	}
	generate(from+1, result, n, startWith)
}

func printMatrix(matrix [][]int) {
	for i := range matrix {
		fmt.Println(matrix[i])
	}
}

func main() {
	fmt.Println("Hello, World!")
	n := 1
	result := generateMatrix(n)
	fmt.Println(result)
}
