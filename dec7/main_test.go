package main

import (
	_ "embed"
	"testing"
)

//go:embed input.txt
var input string

func TestPart1(t *testing.T) {
	var result = Part1(input)
	if result != 95437 {
		t.Errorf(`Wrong result, was %v`, result)
	}
}

func TestPart2(t *testing.T) {
	var result = Part2(input)
	if result != 24933642 {
		t.Errorf(`Wrong result, was %v`, result)
	}
}
