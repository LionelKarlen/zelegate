# Zelegate
> <tagline>

# Vision
<!-- Paragraph on introductory details about the software, inspiration, initial idea. -->
The inbuilt zellij session manager personally feels too clunky for my workflow. This project aims to streamline dealing with zellij sessions, offering a consistent interface for creating, attaching, switching, and managing sessions. The main inspirations are ![Sesh](), ![Sessionizer](), and ![Zellij-Switch]().

<!-- Paragraph on solution overview, what is the software providing. Who is it for? How does it help them? What do we expect the software to do? -->
Zelegate allows you to define custom sessions that can quickly be opened, from wherever you happen to be. Within the same interface it is also possible to attach to a currently running session, or start a new session in a folder, within a set of predefined paths.

# Scope
<!-- What are the fundamental features we need? This is good to rely on to avoid feature drift, and to revist, should the scope change. List potential shortcomings of the planned system, future improvements and whether they could make sense. -->
The core features the software supports are:
1. attaching to existing sessions
1. creating new sessions, based on folders
1. defining custom sessions

This requires a configuration file, the syntax of which is defined in [config.md](config.md). This necessitates additional considerations:
- how is the configuration file accessed?
    > The default location is ~/.config/zelegate/zelegate.toml. The path can also be given by the `--config` flag.
- how does the program function when it isn't found?
    > It simply skips the custom sessions, and folders, and only lists current sessions
    > It runs the default configuration

# User Requirements
<!-- How do our users expect to interact with the software? What do they know, what do they *need* to know? This would be a good place to think about user stories (user a wants to do xzy; they go to abc and do uvw;), and the general hierarchy of the software, any public interface (api, ui). -->
Users can interact with the software directly, using the cli. A full breakdown can be found in the [manual](manual.md). In order to get started though, it suffices to use something like:

```sh
zelegate into $(zelegate list | fzf)
```

# Functional Requirements
<!-- Functional requirements specify the **what** of the system, and can be thought of as the sentence "the system must do <requirement>". This list can become quite large, it would be best to create a hierarchy, where each requirement has a name, a unique number based on the hierarchy, and be accompanied by a description, or implementation detail. -->
The system must:
- List all currently open sessions (#1)
- Attach to a session, or switch to a session (#2)
- Read and parse the configuration file (#3)
- Define custom sessions (#7)
- List all custom sessions (#4)
- Command-agnostic backend for folder sessions (#5)
- List folders (#6)
- Create new sessions (#8)


# Non-functional Requirements
<!-- Non-functional requirements specify the **how** of the system, and can be thought of as the sentence "the system shall (be) <requirement>". They describe the ideals of the program, contracts that should hold, but are not necessarily technical in nature. This portion does not concern itself with implementations. -->
The system shall be:
- Expandable; offer a consistent interface upon which users can create their own workflows
- Deterministic; everything is entirely configurable, and the system is not reliant on internal state

# Visual Requirements
<!-- Provide user interface mock-ups or templates. Include requirements for individual pages. Outline the visual language of the system, colours, themes, icons, imagery. -->
Existing sessions, custom sessions, and folders should have different icons denoting the type of session. This is not a hard requirement though, and could potentially make it harder to attach to the correct session.

# Appendix
- [Roadmap](roadmap.md)
