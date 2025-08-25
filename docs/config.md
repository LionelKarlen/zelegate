The default configuration (no file given) is:
```toml
[general]
# remove all exited sessions before starting up
clean_exited=false
# The source of the folder list. You might want to use something like `zoxide query -l`, or `find .... TODO: fill out`
folder_backend=""
```


An example configuration:
```toml
[general]
folder_backend = "zoxide query -l"

[[session]]
name = "nix"
path = "~/.nixfiles"
layout = "default"
```
