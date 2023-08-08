package main

import (
	"fmt"
)

func main() {
	previous := 0
	current := 1
	n := 10
	for n > 1 {
		{
			t := current
			current = previous + current
			previous = t
			n = n - 1
		}
	}
	fmt.Println(current)
}
