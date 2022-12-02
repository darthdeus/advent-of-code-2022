import std/sequtils
from std/strutils import parseInt
import std/algorithm

var elves = newSeq[int]()
var current = 0

while true:
  try:
    var line = readLine(stdin)

    if line == "":
      elves.add(current)
      current = 0
    else:
      let num: int = parseInt(line)
      current += num

  except EOFError:
    break

elves.add(current)

# Part 1
# echo elves.max

# Part 2
elves.sort()
elves.reverse()
echo elves[0..2].foldl(a + b)
