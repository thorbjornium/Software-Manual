# Powershell

Press <kbd>F2</kbd> to toggle prediction.

Open Powershell profile in VScode.  

```Powershell
code $profile   
```

Go to home folder

```Powershell
cd ~   
```

## Find-File

Install the following software:

```Powershell
winget install junegunn.fzf
winget install sharkdp.fd
winget install sharkdp.bat
```

Open your profile in Powershell:

```Powershell
notepad $Profile 
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

Use <kbd>CTRL</kbd> <kbd>+</kbd> <kbd>F</kbd> to search and preview files in colorized syntax.  
