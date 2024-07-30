package main

import (
	"fmt"
	"testing"
)

func Test1(t *testing.T) {
	n := 3
	k := 3
	fmt.Println(getPermutation(n, k))
}

func Test2(t *testing.T) {
	n := 4
	k := 9
	fmt.Println(getPermutation(n, k))
}

func Test3(t *testing.T) {
	n := 3
	k := 1
	fmt.Println(getPermutation(n, k))
}

func Test4(t *testing.T) {
	n := 9
	k := 3456
	fmt.Println(getPermutation(n, k))
}

func Test5(t *testing.T) {
	n := 3
	k := 5
	fmt.Println(getPermutation(n, k))
}

func Test11(t *testing.T) {
	n := 3
	k := 3
	fmt.Println(getPermutation1(n, k))
}

func Test21(t *testing.T) {
	n := 4
	k := 9
	fmt.Println(getPermutation1(n, k))
}

func Test31(t *testing.T) {
	n := 3
	k := 1
	fmt.Println(getPermutation1(n, k))
}

func Test41(t *testing.T) {
	n := 9
	k := 3456
	fmt.Println(getPermutation1(n, k))
}

func Test51(t *testing.T) {
	n := 3
	k := 5
	fmt.Println(getPermutation1(n, k))
}

func TestGetLet(t *testing.T) {
	n := 3
	k := 5
	fmt.Println(getLet(n, k))
}

// func TestGetLetAndK1(t *testing.T) {
// 	n := 3
// 	k := 1
// 	let:= getLetAndK(n, k
// 	fmt.Println(k2, let)
// }

// func TestGetLetAndK2(t *testing.T) {
// 	n := 5
// 	k := 4
// 	let:= getLetAndK(n, k)
// }

// func TestGetLetAndK3(t *testing.T) {
// 	n := 6
// 	k := 1
// 	let := getLetAndK(n, k)
// }
