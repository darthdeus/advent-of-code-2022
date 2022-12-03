package main

import "core:fmt"

fib :: proc(n: int) -> int {
  switch {
    case n < 1:
      return 0

    case n == 1:
      return 1
  }

  return fib(n-1) + fib(n-2)
}

main :: proc() {
  fmt.printf("test: %d %d %d\n", fib(2), fib(5), fib(10))
}
