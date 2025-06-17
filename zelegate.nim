import osproc
import os
import strutils
import strformat
import parseopt
import json

type SessionKind = enum Attach, Create

proc abort_if_empty(s: string) =
    if s == "":
        echo "aborted."
        quit()

proc pick_session(paths: seq[string]): (string, SessionKind)=
    # define fzf display options
    let FZF_UI = "--no-separator --info hidden --highlight-line --color pointer:blue,gutter:-1"
    let path_string = paths.join(" ")

    # pick from active sessions
    let picked_session = execCmdEx(fmt"zellij ls -n -s | fzf --print-query -1 {FZF_UI}")[0]
    abort_if_empty(picked_session)

    let lines = picked_session.splitLines()
    let query = lines[0]
    var selection = fmt"/{lines[1]}" # fmt to ensure we can split it, even if we don't need the path
    var kind = SessionKind.Attach

    # specify that we want to create a new session
    if query == "new" or query == "n":
        kind = SessionKind.Create
        # query directories
        let (tmp_directories, directories_code) = execCmdEx(fmt"fd -td -c never --min-depth 1 --max-depth 1 --base-directory $HOME . {path_string}")

        # exit gracefully if fd fails
        if directories_code != 0:
            echo &"one or more directory not found: {path_string}."
            quit()

        # trim ending newline
        let directories = tmp_directories.rsplit("\n", maxsplit=1)[0]

        let directory = execCmdEx(fmt"echo '{directories}' | fzf {FZF_UI}")[0]
        abort_if_empty(directory)

        # get last folder name and replace spaces with underscore
        selection=directory.rsplit("/",maxsplit=1)[0]

    return (selection, kind)

var ops = initOptParser(shortNoVal = {'v'}, longNoVal = @["version"])
const version = parseJson(staticRead("zelegate.json"))["version"].getStr()
var paths: seq[string] = @[]
while true:
    ops.next()
    case ops.kind:
    of cmdEnd: break
    of cmdShortOption, cmdLongOption:
        case ops.key:
        of "v","version":
            echo version
            quit()
        else:continue
    of cmdArgument:
        paths.add(ops.key)

if paths.len == 0:
    echo "no paths specified."
    quit()

let (session_path, session_kind) = pick_session(paths)
let session_name = session_path.rsplit("/", maxsplit=1)[1].replace(" ","_")

let is_zellij = getEnv("ZELLIJ")
var cmd = ""
if is_zellij == "0":
    cmd = fmt"""zellij pipe --plugin zellij-switch -- "--session {session_name} --cwd $HOME/{session_path}" """
else:
    cmd = fmt"zellij attach -c {session_name}"

# if we're creating a session, cd to it first
if session_kind == SessionKind.Create:
    let _ = execCmd(fmt"""(cd "$HOME/{session_path}" && {cmd})""")
else:
    let _ = execCmd(cmd)
