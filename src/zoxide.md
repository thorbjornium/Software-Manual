# zoxide

## Installation

```Powershell

winget install ajeetdsouza.zoxide

```

Add this to the end of your $profile in PowerShell:

```PowerShell

Invoke-Expression (& { (zoxide init powershell | Out-String) })

```

See the [zoxide documentation](https://github.com/ajeetdsouza/zoxide) for more information

## Getting started

|Command            |Description    |
|:---               |:---|
|z foo              |cd into highest ranked directory matching foo|
|z foo bar          |cd into highest ranked directory matching foo and bar|
|z foo /            |cd into a subdirectory starting with foo|
|z ~/foo            |z also works like a regular cd command|  
|z foo/             |cd into relative path|  
|z ..               |cd one level up|  
|z -                |cd into previous directory|  
|zi foo             | cd with interactive selection (using fzf)|
|z foo <SPACE><TAB> |show interactive completions (zoxide v0.8.0+, bash 4.4+/fish/zsh only)|

## Configuration

Configuration
Flags

When calling zoxide init, the following flags are available:

    --cmd
        Changes the prefix of the z and zi commands.
        --cmd j would change the commands to (j, ji).
        --cmd cd would replace the cd command.

    --hook <HOOK>

        Changes how often zoxide increments a directory's score:
        Hook    Description
        none    Never
        prompt  At every shell prompt
        pwd (default)   Whenever the directory is changed

    --no-cmd
        Prevents zoxide from defining the z and zi commands.
        These functions will still be available in your shell as __zoxide_z 
        and __zoxide_zi, should you choose to redefine them.

Environment variables

Environment variables can be used for configuration. They must be set before zoxide init is called.

    _ZO_DATA_DIR

        Specifies the directory in which the database is stored.

        The default value varies across OSes:
        OS               Path               Example
        Linux / BSD    $XDG_DATA_HOME or $HOME/.local/share     /home/alice/.local/share
        macOS   $HOME/Library/Application Support   /Users/Alice/Library/Application Support
        Windows    %LOCALAPPDATA%    C:\Users\Alice\AppData\Local

    _ZO_ECHO
        When set to 1, z will print the matched directory before navigating to it.

    _ZO_EXCLUDE_DIRS

        Excludes the specified directories from the database.

        This is provided as a list of globs, separated by OS-specific characters:
        OS      Separator     Example
        Linux / macOS / BSD   :    $HOME:$HOME/private/*
        Windows     ;    $HOME;$HOME/private/*

        By default, this is set to "$HOME".

    _ZO_FZF_OPTS
        Custom options to pass to fzf during interactive selection.
        See man fzf for the list of options.

    _ZO_MAXAGE
        Configures the aging algorithm, which limits the maximum number
        of entries in the database.
        By default, this is set to 10000.

    _ZO_RESOLVE_SYMLINKS
        When set to 1, z will resolve symlinks before adding directories to the database.

## Set environment variables

Assuming you want to set the _ZO_ECHO variable to 1:

Add this to your $profile in PowerShell:

`env:_ZO_ECHO = '1'`

## Help

Usage:
    `zoxide.exe <COMMAND>`

|Commands|Description|
|:---    |:---|
|add     |Add a new directory or increment its rank|
|edit    |Edit the database|
|import  |Import entries from another application|
|init    |Generate shell configuration|
|query   |Search for a directory in the database|
|remove  |Remove a directory from the database|

<br>

|Options|Description|
|:---   |:---|
|-h, --help     |Print help|
|-V, --version  |Print version|

<br>

|Environment variables |Description|
|:---                  |:---|
|_ZO_DATA_DIR          |Path for zoxide data files|
|_ZO_ECHO              |Print the matched directory before navigating to it when set to 1|
|_ZO_EXCLUDE_DIRS      |List of directory globs to be excluded|
|_ZO_FZF_OPTS          |Custom flags to pass to fzf|
|_ZO_MAXAGE            |Maximum total age after which entries start getting deleted|
|_ZO_RESOLVE_SYMLINKS  |Resolve symlinks when storing paths|
