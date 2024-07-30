package main

import (
	"fmt"
)

// Definition for singly-linked list.

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) (*ListNode, int) {
	value := []int{}
	for head != nil {
		value = append([]int{head.Val}, value...)
		head = head.Next
	}
	return helper(value), len(value)
}

func rotateRight(head *ListNode, k int) *ListNode {
	tail, l := reverseList(head)
	if l == 0 {
		return head
	}
	head = tail
	for head.Next != nil {
		head = head.Next
	}
	k = k % l
	for k > 0 {
		head.Next = tail
		head = tail
		tail = tail.Next
		head.Next = nil
		k--
	}
	n, _ := reverseList(tail)
	return n
}

func helper(head []int) *ListNode {
	list := &ListNode{}
	current := list
	for i := 0; i < len(head); i++ {
		current.Next = &ListNode{Val: head[i]}
		current = current.Next
	}
	return list.Next
}

func rotateRight1(head *ListNode, k int) *ListNode {
	list := head
	listCount := head
	count := 0

	for listCount != nil {
		count++
		listCount = listCount.Next
	}

	if count == 0 || k == 0 {
		return head
	}

	for i := 0; i < k%count; i++ {
		list = rotateOneRight(list)
	}
	return list
}

func rotateOneRight(head *ListNode) *ListNode {
	list := head
	start := list

	for list != nil && list.Next != nil {
		if list.Next.Next == nil {
			list.Next.Next = start
			returnValue := list.Next
			list.Next = nil
			return returnValue
		}
		list = list.Next
	}
	return head
}

func printList(head *ListNode) {
	for head != nil {
		fmt.Printf("%d -> ", head.Val)
		head = head.Next
	}
}

func main() {
	fmt.Println("Hello, world!")
}
