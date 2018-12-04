package main

import (
	"fmt"
	"bufio"
	"time"
	"regexp"
	"strconv"
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
	square := [1001][1001]int{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		re := regexp.MustCompile(`#(\d+) @ (\d+),(\d+): (\d+)x(\d+)`)
		match := re.FindStringSubmatch(scanner.Text())
		_, x, y, xOff, yOff := getIds(match)
		for i := x; i < x + xOff; i++ {
			for j := y; j < y + yOff; j++ {
				square[i][j]++
			}
		}
	}

	var count int
	for i := 0; i < len(square); i++ {
		for j := 0; j < len(square[i]); j++ {
			if square[i][j] > 1 {
				count++
			}
		}
	}

	p1 := time.Since(start)
	fmt.Printf("part 1: %v\n", count)
	fmt.Printf("--- %v ---\n\n", p1)

	// part 2
	start = time.Now()
	_, _ = file.Seek(0, 0)
	scanner = bufio.NewScanner(file)

	var id int
	for scanner.Scan() {
		re := regexp.MustCompile(`#(\d+) @ (\d+),(\d+): (\d+)x(\d+)`)
		match := re.FindStringSubmatch(scanner.Text())
		curr, x, y, xOff, yOff := getIds(match)

		var skip bool
		for i := x; i < x + xOff; i++ {
			for j := y; j < y + yOff; j++ {
				if square[i][j] > 1 {
					skip = true
					break
				}
			}
			if skip {
				break
			}
		}
		if !skip {
			id = curr
			break
		}
	}

	p2 := time.Since(start)
	fmt.Printf("part 2: %v\n", id)
	fmt.Printf("--- %v ---\n", p2)
}

func getIds (match []string) (id, posx, posy, posxOffset, posyOffset int) {
	id, _ = strconv.Atoi(match[1])
	posx, _ = strconv.Atoi(match[2])
	posy, _ = strconv.Atoi(match[3])
	posxOffset, _ = strconv.Atoi(match[4])
	posyOffset, _ = strconv.Atoi(match[5])
	return
}