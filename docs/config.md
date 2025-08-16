The default configuration (no file given) is:
```toml
[general]
# Should custom sessions be prioritised in case there is a naming overlap with a folder
prefer_custom = true
# Should creating a new session be prioritised over resurrecting a session, if they have the same name/are the same session
prefer_create = true
# How should a custom session be listed, if it is already running. One of "both" (show both), "custom" (show only the custom session), or "alive" (show only the running session);
alive_custom_behaviour = "alive"
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
