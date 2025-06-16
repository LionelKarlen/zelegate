import osproc
import os
import strutils
import strformat
import parseopt

proc abort_if_empty(s: string) =
    if s == "":
        echo "aborted."
        quit()

proc pick_session(paths: seq[string]): string =
    # define fzf display options
    let FZF_UI = "--no-separator --info hidden --highlight-line --color pointer:blue,gutter:-1"
    let path_string = paths.join(" ")

    # pick from active sessions
    let picked_session = execCmdEx(fmt"zellij ls -n -s | fzf --print-query -1 {FZF_UI}")[0]
    abort_if_empty(picked_session)

    let lines = picked_session.splitLines()
    let query = lines[0]
    var selection = lines[1]

    # specify that we want to create a new session
    if query == "new" or query == "n":
        # query directories
        let (tmp_directories, directories_code) = execCmdEx(fmt"fd -td -c never --min-depth 1 --max-depth 1 --base-directory $HOME . {path_string}")

        # exit gracefully if fd fails
        if directories_code != 0:
            echo &"one or more directory not found: {path_string}."
            quit()

        # trim ending newline
        let directories = tmp_directories.rsplit("\n", maxsplit=1)[0]

        let (directory, exit_code) = execCmdEx(fmt"echo '{directories}' | fzf {FZF_UI}")
        abort_if_empty(directory)

        # get last folder name and replace spaces with underscore
        let name = directory.rsplit("/", maxsplit=2)[1].replace(" ","_")
        selection=name
    return selection

var ops = initOptParser()
var paths: seq[string] = @[]
while true:
    ops.next()
    case ops.kind:
    of cmdEnd: break
    of cmdArgument:
        paths.add(ops.key)
    else:continue
if paths.len == 0:
    echo "no paths specified."
    quit()

let session = pick_session(paths)
echo fmt"session: {session}"

let is_zellij = getEnv("ZELLIJ")
if is_zellij == "0":
    let _ = execCmd(fmt"zellij pipe --plugin zellij-switch -- '--session {session}'")
else:
    let _ = execCmd(fmt"zellij attach -c {session}")
