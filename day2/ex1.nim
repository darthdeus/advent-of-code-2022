import std/sequtils
import std/strutils
import std/algorithm

var data = newSeq[seq[string]]()

while true:
  try:
    var line = readLine(stdin)
    let a, b = line.split(" ")

    data.add(@[a, b])


  except EOFError:
    break

echo data
