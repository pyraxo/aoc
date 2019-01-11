package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"time"
)

var grid [][]string
var carts map[int]*Cart

type Cart struct {
	x, y   int
	moving int
	turn   int
}

func (c *Cart) GetID() int {
	return c.y*1000 + c.x
}

func (c *Cart) MoveNextTick() (coords []int) {
	origID := c.GetID()

	switch c.moving {
	case 1: // UP
		c.y--
		if carts[c.GetID()] != nil {
			coords = []int{c.x, c.y}
			break
		}
		above := string(grid[c.x][c.y])
		switch above {
		case "|":
		case "/":
			c.moving = 4
		case "\\":
			c.moving = 3
		case "+":
			if c.turn%3 == 0 {
				c.moving = 3
			} else if c.turn%3 == 2 {
				c.moving = 4
			}
			c.turn++
		}
	case 2: // DOWN
		c.y++
		if carts[c.GetID()] != nil {
			coords = []int{c.x, c.y}
			break
		}
		below := string(grid[c.x][c.y])
		switch below {
		case "|":
		case "/":
			c.moving = 3
		case "\\":
			c.moving = 4
		case "+":
			if c.turn%3 == 0 {
				c.moving = 4
			} else if c.turn%3 == 2 {
				c.moving = 3
			}
			c.turn++
		}
	case 3: // LEFT
		c.x--
		if carts[c.GetID()] != nil {
			coords = []int{c.x, c.y}
			break
		}
		left := string(grid[c.x][c.y])
		switch left {
		case "-":
		case "/":
			c.moving = 2
		case "\\":
			c.moving = 1
		case "+":
			if c.turn%3 == 0 {
				c.moving = 2
			} else if c.turn%3 == 2 {
				c.moving = 1
			}
			c.turn++
		}
	case 4: // RIGHT
		c.x++
		if carts[c.GetID()] != nil {
			coords = []int{c.x, c.y}
			break
		}
		right := string(grid[c.x][c.y])
		switch right {
		case "-":
		case "/":
			c.moving = 1
		case "\\":
			c.moving = 2
		case "+":
			if c.turn%3 == 0 {
				c.moving = 1
			} else if c.turn%3 == 2 {
				c.moving = 2
			}
			c.turn++
		}
	}

	newID := c.GetID()
	carts[newID] = c
	delete(carts, origID)

	if coords != nil {
		delete(carts, newID)
		return coords
	}

	return nil
}

func main() {
	file, _ := os.Open("./input.txt")
	defer file.Close()

	var p1, p2 time.Duration

	// part 1
	start := time.Now()
	scanner := bufio.NewScanner(file)

	var line int
	grid = make([][]string, 300)
	for i := range grid {
		grid[i] = make([]string, 300)
	}

	carts = make(map[int]*Cart)

	for scanner.Scan() {
		text := scanner.Text()
		for i, char := range text {
			track := string(char)
			if track == "v" || track == "^" || track == "<" || track == ">" {
				mov := 1
				switch track {
				case "^":
				case "v":
					mov = 2
				case "<":
					mov = 3
				case ">":
					mov = 4
				}
				grid[i][line] = "|"
				cart := &Cart{
					x:      i,
					y:      line,
					moving: mov,
				}
				carts[cart.GetID()] = cart
			} else {
				grid[i][line] = track
			}
		}
		line++
	}

	var coords, first, last []int
	for {
		keys := make([]int, len(carts))
		var j int
		for i := range carts {
			keys[j] = i
			j++
		}
		sort.Ints(keys)

		var lastid int
		for _, k := range keys {
			if carts[k] == nil {
				continue
			}
			coords = carts[k].MoveNextTick()
			if coords != nil {
				if first == nil {
					first = coords
					p1 = time.Since(start)
				}
			} else if len(keys) == 1 {
				lastid = k
				break
			}
		}
		if first != nil && lastid != 0 {
			x := lastid % 1000
			y := lastid / 1000
			last = []int{x, y}
			p2 = time.Since(start)
			break
		}
	}

	fmt.Printf("part 1: %v,%v\n", first[0], first[1])
	fmt.Printf("--- %v ---\n", p1)
	fmt.Printf("part 2: %v,%v\n", last[0], last[1])
	fmt.Printf("--- %v ---\n", p2)
}
