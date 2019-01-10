package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"
)

func countPlants(str string, cur int) (total int) {
	for i, char := range str {
		if char == '#' {
			total += (i - cur)
		}
	}
	return
}

func main() {
	file, _ := os.Open("./input.txt")
	defer file.Close()

	// part 1
	start := time.Now()
	scanner := bufio.NewScanner(file)

	var state string
	scanner.Scan()
	state = scanner.Text()[15:]
	scanner.Scan()

	editMap := map[string]rune{}
	for scanner.Scan() {
		text := scanner.Text()

		old := text[2]
		new := text[9]
		if old == new {
			continue
		}

		pre := text[0:5]
		editMap[pre] = rune(new)
	}

	var cur int
	state = "..." + state + "..."
	cur += 3
	for gen := 0; gen < 20; gen++ {
		state = state + "..."

		swapList := map[int]rune{}
		for i := 0; i < len(state)-4; i++ {
			for pre, char := range editMap {
				if state[i:i+5] == pre {
					swapList[i+2] = char
				}
			}
		}

		stateRunes := []rune(state)
		for i, char := range swapList {
			stateRunes[i] = char
		}
		state = string(stateRunes)
	}

	total := countPlants(state, cur)

	elapsed := time.Since(start)
	fmt.Printf("part 1: %v\n", total)
	fmt.Printf("--- %v ---\n", elapsed)

	// part 2
	start = time.Now()
	for gen := 0; gen < 1980; gen++ {
		if strings.HasSuffix(state, "##...") {
			state = state + "..."
		}

		swapList := map[int]rune{}
		for i := 0; i < len(state)-4; i++ {
			for pre, char := range editMap {
				if state[i:i+5] == pre {
					swapList[i+2] = char
				}
			}
		}

		stateRunes := []rune(state)
		for i, char := range swapList {
			stateRunes[i] = char
		}
		state = string(stateRunes)

		// fmt.Println(countPlants(state, cur))
	}

	bigTotal := countPlants(state, cur) + (50000000000-2000)*69

	elapsed = time.Since(start)
	fmt.Printf("part 2: %v\n", bigTotal)
	fmt.Printf("--- %v ---\n", elapsed)
}
