package main

import (
	"fmt"
	"bufio"
	"strings"
	"time"
	"os"
)

func check (e error) {
	if e != nil {
		fmt.Println(e)
	}
}

var (
	two int
	three int
	isTwo bool
	isThree bool
	total int
)


func main () {
	// part 1
	start := time.Now()
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		for _, char := range line {
			num := strings.Count(line, string(char))
			if num >= 2 {
				if num == 2 {
					isTwo = true
				}
				if num == 3 {
					isThree = true
				}
				if isTwo && isThree {
					break
				}
			}
		}
		if isTwo {
			isTwo = false
			two++
		}
		if isThree {
			isThree = false
			three++
		}
	}

	total = two * three
	fmt.Println(total)
	fmt.Printf("--- %v ---\n", time.Since(start))
}