# Powershell 7

<!-- toc -->

<br>  

## Basic handling of Powershell  

Press <kbd>F1</kbd> after typing a command to get help.

Press <kbd>F2</kbd> to toggle prediction.  

Press <kbd>Tab</kbd> to move to or list the next suggestion.  

Press <kbd>Shift</kbd> + <kbd>Tab</kbd> to move to the previous suggestion.  

Press <kbd>→</kbd> to accept the suggestion.  

Press <kbd>Ctrl</kbd> + <kbd>Space</kbd> to list autocomplete suggestions.

Find the Powershell profile `echo $profile` or just `$profile`  

Open the Powershell profile in notepad `notepad $profile`  

Print the current PSReadLine key handlers `Get-PSReadLineKeyHandler`  

or press <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>?</kbd>  

## Must-have tools for Powershell

[zoxide](https://github.com/ajeetdsouza/zoxide) jumps straight to directories.

[intelli-shell](https://github.com/lasantosr/intelli-shell) bookmark commands and includes [tldr](https://github.com/tldr-pages/tldr).

[ripgrep](https://github.com/BurntSushi/ripgrep) - A fast file search tool.  

[igrep](https://github.com/konradsz/igrep) - An alternate grep tool.

[fzf](https://github.com/junegunn/fzf) - A command-line fuzzy finder.  

[fd](https://github.com/sharkdp/fd) - A simple, fast and user-friendly alternative to find.  

[bat](https://github.com/sharkdp/bat) - A cat(1) clone with wings. Features colored output, syntax highlighting, grid view, and more.  

## VS Code options

`code -w` - Wait for the files to be closed before returning.

`code -n` - Force to open a new window.  

`code .` - Opens a new empty window.

`code -r` - Reuse window. Force to open in an already opened window.  

`code -a` - Add folder(s) to the last active window.  

`Get-ChildItem | code -` - Opens the output of the command in VS Code.  

## Misc

Find the file location of the Powershell history file. Edit and remove inaccurate predictions.  

```Powershell
Get-PSReadLineOption | select-Object -Property HistorySavePath
# or
(Get-PSReadLineOption).HistorySavePath
```

## <p style="text-align:center;">Basic Commands</p>

|Command|Description|  
|:---|:---|
|pwd | Prints the current working directory.  |  
|ls  |Lists the contents of the current directory.  |  
|cd  |Changes the current directory.  |  
|md|  Creates a new directory.  |  
|rm  |Removes a file or directory.  |  
|cp  |Copies a file or directory.  |  
|mv  |Moves a file or directory.  |  
|cat | Displays the contents of a file.  |  
|tree | Lists the contents of a directory in a treelike format. |  
|echo | Prints text to the console.  |  
|read | Reads input from the user.  |  
|cls | Clears the console.|  
|exit | Exits the current shell.  |  
|help | Displays information about the builtin commands.  |  
|get-help|  Displays information about a specific command.  |  
|file | Displays information about a file.  |  
|type | Displays the contents of a file.  |  
