package main

import "fmt"

func uniquePaths(m int, n int) int {
	return get1(m+n-2, m-1)
}

func get1(N int, M int) int {
	MAX := 0
	MIN := 0
	if M > N-M {
		MAX = M
		MIN = N - M
	} else {
		MAX = N - M
		MIN = M
	}
	T := 1
	FAC := 1
	i := MAX + 1
	j := 1
	for i <= N || j <= MIN {
		if i <= N {
			T *= i
		}
		if j <= MIN {
			if T%FAC == 0 {
				T /= FAC
				FAC = 1
			} else {
				FAC *= j
			}
		}
		i++
		j++
	}

	// for i := MAX + 1; i <= N; i++ {
	// 	T *= i
	// }
	// FAC := 1
	// for i := 1; i <= MIN; i++ {
	// 	FAC *= i
	// }
	return T / FAC
}

func get(N int, M int) int {
	facN := 1
	facM := 1
	facNM := 1
	if M > N-M {
		for i := 1; i <= N-M; i++ {
			facNM *= i
		}
		facM = facNM
		for i := N - M + 1; i <= M; i++ {
			facM *= i
		}
		facN = facM
		for i := M + 1; i <= N; i++ {
			facN *= i
		}
	} else {
		for i := 1; i <= M; i++ {
			facM *= i
		}
		facNM = facM
		for i := M + 1; i <= N-M; i++ {
			facNM *= i
		}
		facN = facNM
		for i := N - M + 1; i <= N; i++ {
			facN *= i
		}
	}
	return facN / facNM / facM
}

var cache map[string]int

func uniquePaths1(m int, n int) int {
	if cache == nil {
		cache = map[string]int{}
	}

	if m == 1 && n == 1 {
		return 1
	}

	key := fmt.Sprintf("%d-%d", m, n)
	if v, k := cache[key]; k {
		return v
	}

	r := 0
	if n-1 > 0 {
		r = uniquePaths(m, n-1)
	}

	b := 0
	if m-1 > 0 {
		b = uniquePaths(m-1, n)
	}
	cache[key] = r + b

	return r + b
}

func main() {
	fmt.Println("Hello, world!")
}
