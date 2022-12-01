import std/sequtils
# import std/sugar
from std/strutils import parseInt
import std/algorithm

# var elves: seq[seq[int]] = @[]
# var current: seq[int] = @[]

var elves = newSeq[int]()
var current = 0

while true:
  try:
    var line = readLine(stdin)

    if line == "":
      elves.add(current)
      current = 0
      # current = @[]
    else:
      let num: int = parseInt(line)
      # current.add(num)
      current += num

  except EOFError:
    break

elves.add(current)
elves.sort()
elves.reverse()
# echo elves[0..2]
echo elves[0..2].foldl(a + b)

# echo elves.max
# echo elves
