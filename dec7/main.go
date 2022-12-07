package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

type File struct {
	name string
	size int
}

type Dir struct {
	name     string
	files    []*File
	children []*Dir
	size     int
}

func parse(input string) *Dir {
	var stack []*Dir
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "$ cd") {
			dirStr := line[5:]
			if dirStr == ".." {
				stack = stack[:len(stack)-1] // pop
			} else {
				// new directory
				newDir := &Dir{name: dirStr}
				if dirStr == "/" { // Root
					stack = append(stack, newDir)
				} else {
					stack[len(stack)-1].children = append(stack[len(stack)-1].children, newDir)
				}
				stack = append(stack, newDir)
			}
		} else if line != "$ ls" && !strings.HasPrefix(line, "dir ") {
			parts := strings.Split(line, " ")
			size, _ := strconv.Atoi(parts[0])
			file := &File{name: parts[1], size: size}
			stack[len(stack)-1].files = append(stack[len(stack)-1].files, file)
		}
	}

	calcSizes(stack[0])
	return stack[0]
}

func calcSizes(dir *Dir) int {
	for _, file := range dir.files {
		dir.size += file.size
	}
	if len(dir.children) > 0 {
		for _, child := range dir.children {
			dir.size += calcSizes(child)
		}
	}
	return dir.size
}

func sumAllDirsAbove10(dir *Dir) int {
	sum := 0
	if dir.size <= 100000 {
		sum = dir.size
	}
	if len(dir.children) > 0 {
		for _, child := range dir.children {
			sum += sumAllDirsAbove10(child)
		}
	}
	return sum
}

func Part1(input string) int {
	root := parse(input)
	return sumAllDirsAbove10(root)
}

func smallestDirAbove(limit int, smallestSize int, dir *Dir) int {
	newSmallestSize := smallestSize
	if dir.size < smallestSize && dir.size > limit {
		newSmallestSize = dir.size
	}

	if len(dir.children) > 0 {
		for _, child := range dir.children {
			newSmallestSize = smallestDirAbove(limit, newSmallestSize, child)
		}
	}
	return newSmallestSize
}

func Part2(input string) int {
	root := parse(input)
	spaceToDelete := 30000000 - (70000000 - root.size)
	return smallestDirAbove(spaceToDelete, 70000000, root)
}

//go:embed input_full.txt
var inputFull string

func main() {
	fmt.Printf("Part1: %d", Part1(inputFull))
	fmt.Printf("Part2: %d", Part2(inputFull))
}
