# glow

## Markdown viewer

Use it to discover markdown files, read documentation directly on the command line. Glow will find local markdown files in subdirectories or a local Git repository.

![Glow](.\pics\glow.jpg)

More info at [glow](https://github.com/charmbracelet/glow)

Install:

```powershell
winget install charmbracelet.glow
```

Just type `glow` in the terminal and `?` to get help.

A plugin for yazi can be found [here](https://github.com/Reledia/glow.yazi)

Install it with

```powershell
ya pack -a Reledia/glow
```

and add this to yazi.toml:

```toml
[plugin]
prepend_previewers = [
  { name = "*.md", run = "glow" },
]
```
