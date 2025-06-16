# Zelegate
> delegate your zellij sessions

## Why?
I find that the zellij session manager causes me too much friction most of the time. This nim script is a quick way around that, it offers a consistent interface to create new and attach to existing zellij sessions, from both the terminal and inside a zellij session. It's a glorified sh script but it works and, to me, feels better to use.

## Features
- consistent interface for attaching to and creating new sessions
- usable inside a zellij session to switch to a different one
- integrated sessioniser when creating new sessions (see [zellij-sessionizer](https://github.com/silicakes/zellij-sessionizer/tree/main) for an explanation)
- fuzzy find over sessions and folders

## Requirements
- [fd](https://github.com/sharkdp/fd)
- [fzf](https://github.com/junegunn/fzf)
- [zellij-switch](https://github.com/mostafaqanbaryan/zellij-switch) (has to be installed and loaded as a [plugin alias](https://zellij.dev/documentation/plugin-aliases.html) named `zellij-switch`)

## Acknowledgements
- everyone listed in the requirements for their fantastic work!
