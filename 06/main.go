package main

import (
	"fmt"
	"bufio"
	"log"
	"time"
	"strings"
	"strconv"
	"os"
)

func check (e error) {
	if e != nil {
		log.Panic(e)
	}
}

func abs (n int) int {
	if n < 0 {
		n = -n
	}
	return n
}

func main () {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()

	// part 1
	start := time.Now()
	scanner := bufio.NewScanner(file)

	coords := map[int][2]int{}
	var p int
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), ", ")
		x, _ := strconv.Atoi(line[0])
		y, _ := strconv.Atoi(line[1])
		coords[p] = [2]int{x, y}
		p++
	}

	closestMap := map[int]int{}
	omitList := map[int]bool{}
	for i := 0; i < 401; i++ {
		for j := 0; j < 401; j++ {
			currNearest := 700
			prevNearest := 700
			var id int
			for k, coord := range coords {
				distance := abs(coord[0] - i) + abs(coord[1] - j)
				if distance <= currNearest {
					prevNearest = currNearest
					currNearest = distance
					id = k
				}
			}
			if prevNearest == currNearest {
				continue
			}
			if i == 0 || i == 350 || j == 0 || j == 350 {
				omitList[id] = true
			}
			closestMap[id]++
		}
	}
	
	var currMax int
	for i := range coords {
		if omitList[i] {
			continue
		}
		if currMax < closestMap[i] {
			currMax = closestMap[i]
		}
	}

	elapsed := time.Since(start)
	fmt.Printf("part 1: %v\n", currMax)
	fmt.Printf("--- %v ---\n", elapsed)

	// part 2
	start = time.Now()
	var count int
	for i := 0; i < 401; i++ {
		for j := 0; j < 401; j++ {
			var distance int
			for _, coord := range coords {
				distance += abs(coord[0] - i) + abs(coord[1] - j)
			}
			if distance < 10000 {
				count++
			}
		}
	}

	elapsed = time.Since(start)
	fmt.Printf("part 2: %v\n", count)
	fmt.Printf("--- %v ---\n", elapsed)
}