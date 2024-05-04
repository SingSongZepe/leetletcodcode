// example_test.go
package main

import (
	"fmt"
	"testing"
)

func Test1(t *testing.T) {
	var s = "Hello World"
	fmt.Println(lengthOfLastWord(s))
}

func Test2(t *testing.T) {
	var s = "   fly me   to   the moon  "
	fmt.Println(lengthOfLastWord(s))
}

func Test3(t *testing.T) {
	var s = "luffy is still joyboy"
	fmt.Println(lengthOfLastWord(s))
}
