# fzf

<!-- toc -->

Install:

{{#tabs global="install" }}
{{#tab name="Winget" }}

```Powershell
winget install junegunn.fzf
```  

{{#endtab }}
{{#tab name="Scoop" }}

```Powershell
Scoop install fzf 
```  

{{#endtab }}

{{#endtabs }}

Install [Psfzf](https://github.com/kelleyma49/PSFzf?tab=readme-ov-file#psfzf) after fzf. Quick install
instructions [here](powershell_scripts.md#psfzf).

## Usage


Use arrow keys <kbd>↑</kbd> and <kbd>↓</kbd> to move cursor up and down.
<kbd>Enter</kbd> to select the item, <kbd>Ctrl</kbd> + <kbd>C</kbd> / <kbd>Ctrl</kbd> + <kbd>G</kbd> / <kbd>Esc</kbd> to exit  
On multi-select mode (-m), <kbd>TAB</kbd> and <kbd>Shift</kbd> + <kbd>TAB</kbd> to mark multiple items

Mouse: scroll, click, double-click; shift-click and shift-scroll on multi-select mode  

Press <kbd>Ctrl</kbd> + <kbd>T</kbd> to start PSFzf to select provider paths.  
Press <kbd>Ctrl</kbd> + <kbd>R</kbd> to start PSFzf to select a command in the command history saved by PSReadline.  
PSFzf will insert the command into the current line, but it will not execute the command.  

Press <kbd>Alt</kbd> + <kbd>C</kbd> to start fzf in folder mode. Press <kbd>Enter</kbd> to cd into the selected directory.

A few scripts are [here](./powershell_scripts.md#find-files-and-folders)

More details [here](https://github.com/junegunn/fzf)

## Help

**fzf [options]**

### SEARCH

- `-e, --exact` - Enable exact-match  
- `+x, --no-extended` - Disable extended-search mode  
- `-i, --ignore-case` - Case-insensitive match  
- `+i, --no-ignore-case` - Case-sensitive match  
- `--smart-case` - Smart-case match (default)  
- `--scheme=SCHEME` - Scoring scheme `[default|path|history]`  
- `-n, --nth=N[,..]` - Limit search scope using field index expressions  
- `--with-nth=N[,..]` - Transform line presentation using field index expressions  
- `-d, --delimiter=STR` - Field delimiter regex (default: AWK-style)  
- `+s, --no-sort` - Do not sort results  
- `--literal` - Do not normalize Latin script letters  
- `--tail=NUM` - Maximum number of items to keep in memory  
- `--disabled` - Disable search  
- `--tiebreak=CRI[,..]` - Sorting criteria when scores are tied  
  - `[length|chunk|pathname|begin|end|index]` (default: length)  

### INPUT/OUTPUT

- `--read0` - Read input delimited by ASCII NUL characters  
- `--print0` - Print output delimited by ASCII NUL characters  
- `--ansi` - Enable processing of ANSI color codes  
- `--sync` - Synchronous search for multi-staged filtering  

### GLOBAL STYLE

- `--style=PRESET` - Apply a style preset `[default|minimal|full[:BORDER_STYLE]`  
- `--color=COLSPEC` - Set base color scheme `[dark|light|16|bw]` and custom colors  
- `--no-color` - Disable colors  
- `--no-bold` - Disable bold text  

### DISPLAY MODE

- `--height=[~]HEIGHT[%]` - Set window height (fullscreen by default)  
- `--min-height=HEIGHT[+]` - Minimum height when using percentages  
- `--tmux[=OPTS]` - Start `fzf` in a tmux popup (requires tmux 3.3+)  

### LAYOUT

- `--layout=LAYOUT` - Choose layout `[default|reverse|reverse-list]`  
- `--margin=MARGIN` - Set screen margin  
- `--padding=PADDING` - Set padding inside the border  
- `--border[=STYLE]` - Draw border `[rounded|sharp|bold|block|thinblock|double|horizontal|vertical|top|bottom|left|right|none]` (default: `rounded`)  
- `--border-label=LABEL` - Label for the border  
- `--border-label-pos=COL` - Border label position  

### LIST SECTION

- `-m, --multi[=MAX]` - Enable multi-select  
- `--highlight-line` - Highlight the current line  
- `--cycle` - Enable cyclic scroll  
- `--wrap` - Enable line wrapping  
- `--gap[=N]` - Render empty lines between items  
- `--scroll-off=LINES` - Number of lines to keep above/below during scroll  

### INPUT SECTION

- `--no-input` - Hide the input section  
- `--prompt=STR` - Set input prompt (default: `> `)  
- `--info=STYLE` - Set finder info style `[default|right|hidden|inline[-right][:PREFIX]]`  

### PREVIEW WINDOW

- `--preview=COMMAND` - Command to preview highlighted line `{}`  
- `--preview-window=OPT` - Preview window layout (default: `right:50%`)  

### HEADER

- `--header=STR` - Print a header string  
- `--header-lines=N` - Treat the first `N` lines as a header  
- `--header-border[=STYLE]` - Border around the header section  

### SCRIPTING

- `-q, --query=STR` - Start with the given query  
- `-1, --select-1` - Auto-select the only match  
- `-0, --exit-0` - Exit immediately if no match  

### KEY/EVENT BINDING

- `--bind=BINDINGS` - Define custom key/event bindings  

### ADVANCED

- `--with-shell=STR` - Specify shell command for child processes  
- `--listen[=[ADDR:]PORT]` - Start HTTP server for actions  

### DIRECTORY TRAVERSAL

- `--walker=OPTS` - Set traversal options `[file][,dir][,follow][,hidden]`  
- `--walker-root=DIR [...]` - Set root directories (default: `.`)  

### HISTORY

- `--history=FILE` - Specify history file  
- `--history-size=N` - Max number of history entries (default: `1000`)  

### SHELL INTEGRATION

- `--bash` - Setup script for Bash integration  
- `--zsh` - Setup script for Zsh integration  
- `--fish` - Setup script for Fish integration  

### HELP

- `--version` - Display version information  
- `--help` - Show help message  
- `--man` - Show man page  

### ENVIRONMENT VARIABLES

- `FZF_DEFAULT_COMMAND` - Default command when input is `tty`  
- `FZF_DEFAULT_OPTS` - Default options (e.g. `'--layout=reverse --info=inline'`)  
- `FZF_DEFAULT_OPTS_FILE` - File to read default options from  
- `FZF_API_KEY` - API key for HTTP server (`--listen`)  
