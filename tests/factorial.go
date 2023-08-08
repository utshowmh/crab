package main

import (
	"fmt"
)

func main() {
	factorial := 1
	for i := 1; 1 < 5; i++ {
		factorial = factorial * i
	}
	fmt.Println(factorial)
}
