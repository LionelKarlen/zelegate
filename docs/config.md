The default configuration (no file given) is:
```toml
[general]
# Should custom sessions be prioritised in case there is a naming overlap with a folder
prefer_custom = true
# Should creating a new session be prioritised over resurrecting a session, if they have the same name/are the same session
prefer_create = true
# The source of the folder list. You might want to use something like `zoxide query -l`, or `find .... TODO: fill out`
folder_backend=""

[visual]
# Icon for the currently active session
active = ""
# Icon for all running sessions
alive= ""
# Icon for exited, but ressurectable sessions
exited = ""
# Icon for custom sessions
custom = ""
# Icon for folders
folder = ""
```


An example configuration:
```toml
[general]
folder_backend = "zoxide query -l"

[visual]
icon = ""

[[session]]
name = "nix"
path = "~/.nixfiles"
layout = "default"
```
