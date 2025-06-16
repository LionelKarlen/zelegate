import os
import osproc
import strutils
import strformat
import parseopt

var ops = initOptParser()
var paths: seq[string] = @[]
while true:
    ops.next()
    case ops.kind:
    of cmdEnd: break
    of cmdArgument:
        paths.add(ops.key)
    else:continue



let is_zellij = getEnv("ZELLIJ")
var cmd = ""
if is_zellij == "0":
    cmd = fmt"""zellij action new-pane -f -c --name zelegate -- zelegate {paths.join(" ")}"""
else:
    cmd = fmt"""zelegate {paths.join(" ")}"""
let _ = execCmd(cmd)
