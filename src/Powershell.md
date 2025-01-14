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

Use <kbd>CTRL</kbd> + <kbd>F</kbd> to search and preview files in colorized syntax.  

## Commands

pwd - Prints the current working directory.  

ls - Lists the contents of the current directory.  

cd - Changes the current directory.  

mkdir - Creates a new directory.  

rm - Removes a file or directory.  

cp - Copies a file or directory.  

mv - Moves a file or directory.  

cat - Displays the contents of a file.  

tree - Lists the contents of a directory in a tree-like format.  

echo - Prints text to the console.  

read - Reads input from the user.  

cls - Clears the console.

exit - Exits the current shell.  

help - Displays information about the built-in commands.  

get-help - Displays information about a specific command.  

get-childitem -path <path> -recurse -filter <filter>  

get-childitem -path <path> -recurse -filter <filter> |  foreach-object { $_.fullname }  

get-childitem -path <path> -recurse -filter <filter> | foreach-object { $_.fullname } | fzf   

get-childitem -path <path> -recurse -filter <filter> | foreach-object { $_.fullname } | fzf | foreach-object { code $_ }  

file - Displays information about a file.  

type - Displays the contents of a file.  

type -path <path> -literalpath <literalpath> | clip -no-cpy