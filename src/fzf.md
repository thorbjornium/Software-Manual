# fzf

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
