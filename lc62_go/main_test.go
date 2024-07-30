package main

import (
	"fmt"
	"testing"
)

func Test1(t *testing.T) {
	m := 3
	n := 7
	res := uniquePaths(m, n)
	fmt.Println(res)
}

func Test2(t *testing.T) {
	m := 3
	n := 2
	res := uniquePaths(m, n)
	fmt.Println(res)
}

func Test3(t *testing.T) {
	m := 3
	n := 3
	res := uniquePaths(m, n)
	fmt.Println(res)
}

func Test4(t *testing.T) {
	m := 16
	n := 16
	res := uniquePaths(m, n)
	fmt.Println(res)
}
