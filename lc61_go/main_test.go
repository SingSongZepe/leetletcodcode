package main

import (
	"testing"
)

func Test1(t *testing.T) {
	head := helper([]int{1, 2, 3, 4, 5})
	k := 2
	list := rotateRight(head, k)
	printList(list)
}

func Test2(t *testing.T) {
	head := helper([]int{0, 1, 2})
	k := 4
	list := rotateRight(head, k)
	printList(list)
}

func Test3(t *testing.T) {
	head := helper([]int{0, 1, 2})
	k := 9999998
	list := rotateRight(head, k)
	printList(list)
}

func Test4(t *testing.T) {
	head := helper([]int{})
	k := 0
	list := rotateRight(head, k)
	printList(list)
}

func Test11(t *testing.T) {
	head := helper([]int{1, 2, 3, 4, 5})
	k := 2
	list := rotateRight1(head, k)
	printList(list)
}

func Test21(t *testing.T) {
	head := helper([]int{0, 1, 2})
	k := 4
	list := rotateRight1(head, k)
	printList(list)
}

func Test31(t *testing.T) {
	head := helper([]int{0, 1, 2})
	k := 9999998
	list := rotateRight1(head, k)
	printList(list)
}

func Test41(t *testing.T) {
	head := helper([]int{})
	k := 0
	list := rotateRight1(head, k)
	printList(list)
}
