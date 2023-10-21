# Tmux Greeter
Tmux Greeter is a TUI menu to open directories or run commands in tmux defined in a configuration file.
![Example screnshot](https://raw.githubusercontent.com/plsdontreviveme/tmux-greeter/main/assets/example.png)

The shortcuts are loaded from the file 
```bash
~/.config/tmux-greeter/config.toml
```

Example configuration: 

```toml

[directories]
titles = ["Development", "Picutures", ".Config"]
paths = ["/home/user/Dev", "/home/user/Pictures", "/home/user/.config"]

[executables]
titles = ["Newsboat", "Cmus", "Neovim", "Htop"]
commands = ["newsboat", "cmus", "nvim", "htop"]
```

Tmux Greeter also supports custom themes loaded from
```bash
~/.config/tmux-greeter/theme.toml
```

Example configuration:
```toml
shadow = false
borders = "simple" # Alternatives are "none" and "simple"
[colors]
	background = "default" # default terminal background colour
	view       = "black" # black defined by terminal configuration, must be all lowercase

	# An array with a single value has the same effect as a simple value.
	primary   = "white"
	secondary = "#EEEEEE" # hex codes also work assuming your terminal supports it
	tertiary  = "#252521"
	title_primary   = "yellow"
	title_secondary = "#ffff55"

	# Lower precision values can use only 3 digits.
	highlight          = "#F88"
	highlight_inactive = "#5555FF"

[styles.highlight]
    effects = []
    front = "red"
    back = "inherit_parent"
```
