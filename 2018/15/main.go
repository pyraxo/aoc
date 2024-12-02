package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"time"
)

// X is the width of the grid
const X = 7

// Y is the height of the grid
const Y = 7

var row = []int{-1, 1, 0, 0}
var col = []int{0, 0, 1, -1}

var grid [][]string
var units map[int]*Unit

type Unit struct {
	x, y  int
	hp    int
	group int
}

type Node struct {
	x, y, dist int
}

// GetPriority returns the priority int of a unit
func (u *Unit) GetPriority() int {
	return u.y*1000 + u.x
}

func (u *Unit) GetGroup() string {
	c := []string{"G", "E"}
	return c[u.group]
}

// GetEnemy returns the unit's enemy int
func (u *Unit) GetEnemy() (enemy int) {
	if u.group == 0 {
		enemy = 1
	}
	return
}

func createGrid(x, y int) (grid [][]string) {
	grid = make([][]string, x)
	for i := range grid {
		grid[i] = make([]string, y)
	}
	return
}

func createOutputGrid(x, y int) (o [][]int) {
	o = make([][]int, x)
	for i := range o {
		o[i] = make([]int, y)
	}
	return
}

func isValid(x, y int) bool {
	return x > 0 && x < X && y > 0 && y < Y
}

func isSafe(dist, x, y int, output [][]int) bool {
	return grid[x][y] == "." && units[y*1000+x] == nil && (output[x][y] == -1 || dist < output[x][y])
}

// hasAdjacentEnemy checks if a unit has enemies on adjacent tiles
func hasAdjacentEnemy(currUnit *Unit, units map[int]*Unit) bool {
	enemy := currUnit.GetEnemy()
	for i := 0; i < 4; i++ {
		nx := currUnit.x + row[i]
		ny := currUnit.y + col[i]
		adj := units[ny*1000+nx]
		if adj != nil && adj.group == enemy {
			return true
		}
	}
	return false
}

func main() {
	file, _ := os.Open("./1.txt")
	defer file.Close()

	// part 1
	start := time.Now()
	scanner := bufio.NewScanner(file)

	var line int
	groupCount := [2]int{}
	grid = createGrid(X, Y)
	units = make(map[int]*Unit)

	for scanner.Scan() {
		text := scanner.Text()
		for i := 0; i < len(text); i++ {
			char := string(text[i])
			if char == "G" || char == "E" {
				grid[i][line] = "."
				var group int
				if char == "E" {
					group = 1
				}
				groupCount[group]++
				unit := &Unit{
					x:     i,
					y:     line,
					hp:    200,
					group: group,
				}
				units[unit.GetPriority()] = unit
			} else {
				grid[i][line] = char
			}
		}
		line++
	}

	var round int
	for groupCount[0] > 0 && groupCount[1] > 0 {
		round++
		fmt.Printf("round %v\n", round)

		keys := make([]int, len(units))
		var j int
		for i := range units {
			keys[j] = i
			j++
		}
		sort.Ints(keys)

		for _, k := range keys {
			currUnit := units[k]
			if currUnit == nil {
				continue
			}
			enemyEnum := currUnit.GetEnemy()

			// Movement phase
			if !hasAdjacentEnemy(currUnit, units) {
				// Discovery phase
				output := createOutputGrid(X, Y)
				queue := []Node{}
				for i := 0; i < X; i++ {
					for j := 0; j < Y; j++ {
						output[i][j] = -1
						if units[j*1000+i] == nil {
							continue
						}
						if units[j*1000+i].group == enemyEnum {
							node := Node{i, j, 0}
							queue = append(queue, node)
							output[i][j] = 0
						}
					}
				}

				for len(queue) > 0 {
					var curr Node
					curr, queue = queue[0], queue[1:]
					x := curr.x
					y := curr.y
					dist := curr.dist

					for i := 0; i < 4; i++ {
						nx := x + row[i]
						ny := y + col[i]
						if isSafe(dist+1, nx, ny, output) && isValid(nx, ny) {
							output[nx][ny] = dist + 1
							node := Node{nx, ny, dist + 1}
							queue = append(queue, node)
						}
					}
				}
				/*
					for i := 0; i < len(output); i++ {
						fmt.Println(output[i])
					}
				*/

				// Movement find phase
				var x, y int
				currMin := 100000
				for i := 0; i < 4; i++ {
					nx := currUnit.x + row[i]
					ny := currUnit.y + col[i]
					adjNode := output[nx][ny]
					if units[ny*1000+nx] != nil {
						continue
					}
					if adjNode != -1 && adjNode < currMin {
						currMin = adjNode
						x = nx
						y = ny
					}
				}

				if currMin != 100000 {
					// Movement phase
					fmt.Printf("%v moving from %v,%v to %v,%v\n", currUnit.GetGroup(), currUnit.x, currUnit.y, x, y)
					orig := currUnit.GetPriority()
					currUnit.x = x
					currUnit.y = y
					units[y*1000+x] = currUnit
					delete(units, orig)
				}
			}

			// Attack phase
			lowest := 201
			var pos int
			for i := 0; i < 4; i++ {
				nx := currUnit.x + row[i]
				ny := currUnit.y + col[i]
				adj := units[ny*1000+nx]
				if adj != nil && adj.group == enemyEnum && adj.hp < lowest {
					pos = ny*1000 + nx
					lowest = adj.hp
				}
			}
			if pos != 0 {
				// fmt.Printf("%v at %v,%v attacking %v at %v,%v\n", currUnit.GetGroup(), currUnit.x, currUnit.y, target.GetGroup(), target.x, target.y)
				target := units[pos]
				target.hp -= 3

				if target.hp <= 0 {
					// Death
					fmt.Printf("%v died at %v,%v\n", target.GetGroup(), target.x, target.y)
					delete(units, target.GetPriority())
					groupCount[target.group]--
					fmt.Printf("%vG and %vE left\n", groupCount[0], groupCount[1])
				}
			}
		}
	}

	var totalhp int
	for _, u := range units {
		fmt.Println(u.hp)
		totalhp += u.hp
	}
	result := totalhp * round

	elapsed := time.Since(start)
	fmt.Println(totalhp)
	fmt.Printf("part 1: %v\n", result)
	fmt.Printf("--- %v ---\n", elapsed)
}
