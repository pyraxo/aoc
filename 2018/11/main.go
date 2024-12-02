package main

import (
	"fmt"
	"time"
)

const input int = 5034

func getHundredsPlace(num int) int {
	return (num / 100) % 10
}

func getPowerLevel(x, y int) (power int) {
	rackID := x + 10
	power = rackID * y
	power += input
	power *= rackID
	power = getHundredsPlace(power)
	power -= 5
	return
}

func getSquareTotal(grid [][]int, x, y, size int) (power int) {
	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			power += grid[x+i][y+j]
		}
	}
	return
}

func main() {
	// part 1
	start := time.Now()

	grid := make([][]int, 301)
	for i := range grid {
		grid[i] = make([]int, 301)
	}

	for x := 1; x < 301; x++ {
		for y := 1; y < 301; y++ {
			grid[x][y] = getPowerLevel(x, y)
		}
	}

	var xmax, ymax, maxTotal int
	for i := 1; i < 298; i++ {
		for j := 1; j < 298; j++ {
			total := getSquareTotal(grid, i, j, 3)
			if maxTotal < total {
				maxTotal = total
				xmax = i
				ymax = j
			}
		}
	}

	elapsed := time.Since(start)
	fmt.Printf("part 1: %v,%v\n", xmax, ymax)
	fmt.Printf("--- %v ---\n", elapsed)

	// part 2
	start = time.Now()

	var xmax2, ymax2, maxTotal2, maxSize int
	for size := 1; size < 301; size++ {
		border := 301 - size
		for x := 1; x < border; x++ {
			for y := 1; y < border; y++ {
				total := getSquareTotal(grid, x, y, size)
				if maxTotal2 < total {
					maxTotal2 = total
					xmax2 = x
					ymax2 = y
					maxSize = size
				}
			}
		}
	}

	elapsed = time.Since(start)
	fmt.Printf("part 2: %v,%v,%v\n", xmax2, ymax2, maxSize)
	fmt.Printf("--- %v ---\n", elapsed)
}
