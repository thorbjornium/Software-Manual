# GitHub CLI

<!-- toc -->

GitHub CLI, or gh, is a command-line interface to GitHub and [GitHub-Gist](https://gist.github.com) for use in your terminal or your scripts. Manual is found [here](https://cli.github.com/manual) and the GitHub page is [here](https://github.com/cli/cli).  

Install:

```powershell

winget install --id GitHub.cli

```

Plugins is found [here](https://github.com/topics/gh-extension).  

## Completions  

Add the following to your powershell profile:

```powershell  

Invoke-Expression -Command $(gh completion -s powershell | Out-String)

```

<br>  

## Examples  

### Create a repository  

- create a repository interactively  
`gh repo create`  

- create a new remote repository and clone it locally  
`gh repo create my-project --public --clone`  

- create a new remote repository in a different organization  
`gh repo create my-org/my-project --public`  

- create a remote repository from the current directory  
`gh repo create my-project --private --source=. --remote=upstream`  

### Clone a repository  

- Clone a repository from a specific org  
`gh repo clone cli/cli`  

- Clone a repository from your own account  
`gh repo clone myrepo`  

- Clone a repo, overriding git protocol configuration  
`gh repo clone https://github.com/cli/cli`  
`gh repo clone git@github.com:cli/cli.git`  

- Clone a repository to a custom directory  
`gh repo clone cli/cli workspace/cli`  

- Clone a repository with additional git clone flags  
`gh repo clone cli/cli -- --depth=1`  
