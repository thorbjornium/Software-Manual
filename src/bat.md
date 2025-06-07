# bat

A file previewer with syntax highlighting and git integration.  
bat [homepage](https://github.com/sharkdp/bat)

## Install

{{#tabs global="install" }}
{{#tab name="Winget" }}

```Powershell

winget install sharkdp.bat
winget install jftuga.less

```  

{{#endtab }}
{{#tab name="Scoop" }}

```Powershell

Scoop bat less

```  

{{#endtab }}

{{#endtabs }}

[less](https://www.greenwoodsoftware.com/less/faq.html) is a pager for text files that supports syntax highlighting.

You will need to install the [Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170) package.

Get shell completions by running `bat --completion ps1`

## Usage

Display a single file on the terminal

```Powershell
bat README.md
```

Display multiple files at once

```Powershell
bat *.rs
```

Read from stdin, determine the syntax automatically (note, highlighting will only work if the syntax can be determined from the first line of the file, usually through a shebang such as #!/bin/sh)

```Powershell
curl -s https://sh.rustup.rs | bat
```

Read from stdin, specify the language explicitly

```Powershell
yaml2json .travis.yml | json_pp | bat -l json
```

Show and highlight non-printable characters:

```Powershell
bat -A 'c:\windows\system32\drivers\etc\hosts'
```

Use it as a cat replacement:

```Powershell
bat > note.md  # quickly create a new file. Exit with Ctrl+C
```

```Powershell
bat header.md content.md footer.md > document.md # Concatenate (cat) files into one
```

```Powershell
bat -n main.rs  # show line numbers (only)
```

Integration with other tools
fzf
You can use bat as a previewer for fzf. To do this, use `bat --color=always` option to force colorized output. You can also use `--line-range` option to restrict the load times for long files:

```powershell
fzf --preview "bat --color=always --style=numbers --line-range=:500 {}"
```

For more information, see fzf's README.

`find` or `fd`
You can use the `-exec` option of find to preview all search results with bat:
`find … -exec bat {} +`
If you happen to use `fd`, you can use the `-X/--exec-batch` option to do the same:

`fd … -X bat`

`bat` can be combined with `tail -f` to continuously monitor a given file with syntax highlighting.

```powershell
tail -f .\var\log\pacman.log | bat --paging=never -l log
```

Note that we have to switch off paging in order for this to work. We have also specified the syntax explicitly (-l log), as it can not be auto-detected in this case.

**Git**  
You can combine `bat` with `git show` to view an older version of a given file with proper syntax highlighting:

```powershell
git show v0.6.0:main.rs | bat -l rs
```

`git diff`  
You can combine `bat` with `git diff` to view lines around code changes with proper syntax highlighting:

```powershell
function batdiff() {
    git diff --name-only --relative --diff-filter=d | xargs bat --diff
}
```

`clip`  
The line numbers and Git modification markers in the output of bat can make it hard to copy the contents of a file. To prevent this, you can call bat with the -p/--plain option or simply pipe the output into clip:

```powershell
bat main.rs | clip
```

bat will detect that the output is being redirected and print the plain file contents.
