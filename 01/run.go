package main

import (
	"fmt"
	"bufio"
	"strconv"
	"time"
	"os"
)

func check (e error) {
	if e != nil {
		fmt.Println(e)
	}
}


func main () {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()

	// part 1
	start := time.Now()

	var sum int
	var numList = []int{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		num, err := strconv.Atoi(line)
		check(err)
		numList = append(numList, num)
		sum += num
	}

	elapsed := time.Since(start)
	fmt.Printf("part 1: %v\n", sum)
	fmt.Printf("--- %v ---\n", elapsed)

	// part 2
	start = time.Now()

	freqs := map[int]bool{}
	sum = 0
	for {
		for _, num := range numList {
			sum += num
			if freqs[sum] {
				elapsed = time.Since(start)
				fmt.Printf("part 2: %v\n", sum)
				fmt.Printf("--- %v ---\n", elapsed)
				os.Exit(0)
			} else {
				freqs[sum] = true
			}
		}
	}
}