# Powershell

Press <kbd>F2</kbd> to toggle prediction.  

Press <kbd>Tab</kbd> to move to or list the next suggestion.  

Press <kbd>Shift</kbd> + <kbd>Tab</kbd> to move to the previous suggestion.  

Press <kbd>→</kbd> to accept the suggestion.  

Press <kbd>Ctrl</kbd> + <kbd>Space</kbd> to list autocomplete suggestions.

Open Powershell profile in notepad:  

```Powershell
notepad $profile   
```

Print the current PSReadLine key handlers:

```Powershell  
Get-PSReadLineKeyHandler
```

or press <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>?</kbd>

## Software and a script to enhance Powershell

### Find-File

Install the following software:

```Powershell
winget install junegunn.fzf
winget install sharkdp.fd
winget install sharkdp.bat
```

Open your Powershell profile in VS Code:

```Powershell
code $Profile 
```

Add the following lines at the end of the file:

```Powershell
function Find-File {
    $file_path = fd --type file --follow --exclude .git | 
    fzf --ansi --preview 'bat --color=always {} --style=numbers,changes'

    return $file_path
}

function Invoke-FileAction ($Path) {
    $commands = @{
        "Edit file in Visual Studio Code" = { code $Path }
        "Delete file"                     = { Remove-Item -Recurse -Force $Path }
    }

    $selected_command = $commands.Keys | fzf --prompt "Select action >"
    &$commands[$selected_command]
}

function Invoke-FileFinder() {
    $file = Find-File

    # Make sure we selected a file at all.
    if (!$file) {
        return
    }

    # Make sure we have a valid file path.
    if (Test-Path $file) {
        Invoke-FileAction -Path $file
    }
}

Set-PSReadLineKeyHandler -chord "ctrl+f" -scriptblock {
    [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
    [Microsoft.PowerShell.PSConsoleReadLine]::Insert("Invoke-FileFinder")
    [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
}
```

## Basic Commands

|Command|Description|  
|:---|:---|
|pwd | Prints the current working directory.  |  
|ls  |Lists the contents of the current directory.  |  
|cd  |Changes the current directory.  |  
|mkdir|  Creates a new directory.  |  
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
