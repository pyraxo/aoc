package main

import (
	"fmt"
	"strconv"
	"time"
)

const input int = 147061

var recipes []int
var a, b int

func NextStep() {
	newRecipe := recipes[a] + recipes[b]
	firstDigit := newRecipe / 10
	if firstDigit != 0 {
		recipes = append(recipes, firstDigit)
	}
	recipes = append(recipes, newRecipe%10)

	aOffset := recipes[a] + 1
	a = (aOffset + a) % len(recipes)
	bOffset := recipes[b] + 1
	b = (bOffset + b) % len(recipes)
}

func main() {
	// part 1
	start := time.Now()
	b = 1
	recipes = []int{3, 7}

	for len(recipes) < input+10 {
		NextStep()
	}

	var result string
	for i := input; i < input+10; i++ {
		result += strconv.Itoa(recipes[i])
	}
	p1 := time.Since(start)
	fmt.Printf("part 1: %v\n", result)
	fmt.Printf("--- %v ---\n", p1)

	// part 2
	start = time.Now()

	var recipeCount, cur int
	comp := strconv.Itoa(input)
	cur = len(recipes) - 7
	for {
		NextStep()
		for i := cur; i < len(recipes)-6; i++ {
			var str string
			for j := 0; j < 6; j++ {
				str += strconv.Itoa(recipes[i+j])
			}
			if comp == str {
				recipeCount = i
			} else {
				cur = i
			}
		}
		if recipeCount != 0 {
			break
		}
	}

	p2 := time.Since(start)
	fmt.Printf("part 2: %v\n", recipeCount)
	fmt.Printf("--- %v ---\n", p2)
}
