package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

func altSolution() {
	file, _ := os.Open("./input.txt")
	defer file.Close()

	// part 1
	start := time.Now()

	scanner := bufio.NewScanner(file)
	entries := []string{}
	for scanner.Scan() {
		entries = append(entries, scanner.Text())
	}
	sort.Strings(entries)

	var guard, sleep, wake int
	guards := map[int]map[int]int{}
	for _, str := range entries {
		if strings.Contains(str, "#") {
			guard, _ = strconv.Atoi(strings.Split(str, " ")[3][1:])
			if guards[guard] == nil {
				guards[guard] = map[int]int{}
				guards[guard][-1] = 0
			}
		} else if strings.Contains(str, "falls") {
			sleep, _ = strconv.Atoi(str[15:17])
		} else {
			wake, _ = strconv.Atoi(str[15:17])
			for i := sleep; i <= wake; i++ {
				guards[guard][-1]++
				if guards[guard][i] == 0 {
					guards[guard][i] = 1
				} else {
					guards[guard][i]++
				}
			}
		}
	}

	// Finding highest guard
	var maxTotal, maxGuard int
	for id, guard := range guards {
		count := guard[-1]
		if count > maxTotal {
			maxTotal = count
			maxGuard = id
		}
	}

	// Finding highest count
	var maxCount, maxMin int
	for min, count := range guards[maxGuard] {
		if min == -1 {
			continue
		}
		if count > maxCount {
			maxCount = count
			maxMin = min
		}
	}

	result := maxMin * maxGuard
	elapsed := time.Since(start)
	fmt.Printf("part 1: %v\n", result)
	fmt.Printf("--- %v ---", elapsed)
}
