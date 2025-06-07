# Misc

<!-- toc -->

## Global .gitignore

Set up a global `core.excludesfile` configuration file and point it to a global `.gitignore` file located in your home or user directory. Check if you already have this config:

```sh

git config --get core.excludesfile

```

If you have a .gitignore located in your home or user directory:

**nix or Windows git bash:**

```bash

git config --global core.excludesFile '~/.gitignore' 

```

**Windows cmd:**

```cmd

git config --global core.excludesFile "%USERPROFILE%\.gitignore"

```

**Windows PowerShell:**

```powershell

git config --global core.excludesFile "$Env:USERPROFILE\.gitignore" 

```

For Windows it is set to the location `C:\Users\%username%\.gitignore`. You can verify that the config value is correct by doing:

```powershell

git config --global core.excludesFile 

```

The result should be the expanded path to your user profile's `.gitignore`. Ensure that the value does not contain the unexpanded `%USERPROFILE%` string.

**Important**: The above commands will only set the location of the ignore file that git will use. The file has to still be manually created in that location and populated with the ignore list.
