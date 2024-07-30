package main

import (
	"fmt"
)

func lengthOfLastWord(s string) int {
	count := 0
	first := false
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] == 32 {
			if first {
				return count
			} else {
				continue
			}
		}
		count++
		first = true
	}
	return count
}

func main() {
	fmt.Println("Hello World")
}
