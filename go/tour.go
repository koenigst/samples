package main

import (
	"fmt"
	"math"
)

const (
	Big   = 1 << 100
	Small = Big >> 99
)

func Sqrt(x float64) float64 {
	const eps = 1024 * 1024 * 1024 * 1024 * 1024
	z := x
	for {
		d := (z*z - x) / (2 * z)
		if math.Abs(d) < math.Abs(z)/eps {
			return z
		}
		z -= d
		fmt.Println(z)
	}
}

func main() {
	fmt.Println(math.Pi)
	fmt.Println(Sqrt(2))
	fmt.Println(math.Sqrt(2))
}
