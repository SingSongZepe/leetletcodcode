package main

import (
	"fmt"
	"testing"
)

func Test1(t *testing.T) {
	n := 1
	result := generateMatrix(n)
	fmt.Println(result)
}

func Test2(t *testing.T) {
	n := 2
	result := generateMatrix(n)
	fmt.Println(result)
}

func Test3(t *testing.T) {
	n := 3
	result := generateMatrix(n)
	fmt.Println(result)
}

func Test30(t *testing.T) {
	n := 30
	result := generateMatrix(n)
	//fmt.Println(result)
	printMatrix(result)
}

func TestNum(t *testing.T) {
	dividend := 3
	divisor := 2
	fmt.Println(dividend / divisor)
}
