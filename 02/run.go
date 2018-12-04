package main

import (
	"fmt"
	"bufio"
	"strings"
	"time"
	"log"
	"os"
)

func check (e error) {
	if e != nil {
		log.Fatal(e)
	}
}


func main () {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()

	// part 1
	start := time.Now()

	var (
		two int
		three int
		isTwo bool
		isThree bool
		total int
	)

	var store []string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		store = append(store, line)
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
	p1 := time.Since(start)
	fmt.Printf("part 1: %v\n", total)
	fmt.Printf("--- %v ---\n\n", p1)

	// part 2
	start = time.Now()
	for i, line := range store {
		storeLen := len(store)
		for j := i + 1; j < storeLen; j++ {
			var count int
			var str string
			compStr := store[j]
			compLen := len(compStr)

			for k := 0; k < compLen; k++ {
				if line[k] == compStr[k] {
					str += string(line[k])
				} else {
					count++
				}
				if count == 2 {
					break
				}
			}
			
			if count < 2 {
				p2 := time.Since(start)
				fmt.Printf("part 2: %v\n", str)
				fmt.Printf("--- %v ---\n", p2)
				os.Exit(0)
			}
		}
	}
}