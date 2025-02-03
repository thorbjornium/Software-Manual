# Git  

<!-- toc -->

## Git Commands  

Documentation can be found <a href="https://git-scm.com/docs" target="_blank">here</a></p>  

### <p style="text-align:center;">Git Configuration & Setup</p>  

| Commands                                                | Description                                                                        |
| ------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| git config –global user.name “Your Name”                | Set your username globally.                                                        |
| git config –global user.email “<youremail@example.com>” | Set your email globally.                                                           |
| git config –global color.ui auto –                      | Set to display colored output in the terminal                                      |
| git help                                                | Display the main help documentation, showing a list of commonly used Git commands. |

### <p style="text-align:center;">Initializing a Repository</p>

| Commands                                         | Description                                                          |
| ------------------------------------------------ | -------------------------------------------------------------------- |
| git init                                         | Initializes a new Git repository in the current directory.           |
| git init &lt;directory&gt;                       | Creates a new Git repository in the specified directory.             |
| git clone <repository_url>                       | this Clones a repository from a remote server to your local machine. |
| git clone –branch <branch_name> <repository_url> | Clones a specific branch from a repository.                          |

### <p style="text-align:center;">Basic Git Commands</p>

| Commands                                                                 | Description                                                                                                                                                                         |
| ------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| git add &lt;file&gt;                                                     | Adds a specific file to the staging area.                                                                                                                                           |
| git add . or git add –all                                                | Adds all modified and new files to the staging area.                                                                                                                                |
| git status                                                               | Shows the current state of your repository, including tracked and untracked files, modified files, and branch information.                                                          |
| git status –ignored                                                      | Displays ignored files in addition to the regular status output.                                                                                                                    |
| git diff                                                                 | Shows the changes between the working directory and the staging area (index).                                                                                                       |
| git diff &lt;commit1&gt; &lt;commit2&gt;                                 | Displays the differences between two commits.                                                                                                                                       |
| git diff –staged or git diff –cached                                     | Displays the changes between the staging area (index) and the last commit.                                                                                                          |
| git diff HEAD                                                            | Display the difference between the current directory and the last commit                                                                                                            |
| git commit                                                               | Creates a new commit with the changes in the staging area and opens the default text editor for adding a commit message.                                                            |
| git commit -m “&lt;message&gt;” or git commit –message “&lt;message&gt;”    | Creates a new commit with the changes in the staging area and specifies the commit message inline.                                                                                  |
| git commit -a or git commit –all                                         | Commits all modified and deleted files in the repository without explicitly using git add to stage the changes.                                                                     |
| git notes add                                                            | Creates a new note and associates it with an object (commit, tag, etc.).                                                                                                            |
| git restore &lt;file&gt;                                                 | Restores the file in the working directory to its state in the last commit.                                                                                                         |
| git reset &lt;commit&gt;                                                 | Moves the branch pointer to a specified commit, resetting the staging area and the working directory to match the specified commit.                                                 |
| git reset –soft &lt;commit&gt;                                           | Moves the branch pointer to a specified commit, preserving the changes in the staging area and the working directory.                                                               |
| git reset –hard &lt;commit&gt;                                           | Moves the branch pointer to a specified commit, discarding all changes in the staging area and the working directory, effectively resetting the repository to the specified commit. |
| git rm &lt;file&gt;                                                      | Removes a file from both the working directory and the repository, staging the deletion.                                                                                            |
| git mv                                                                   | Moves or renames a file or directory in your Git repository.                                                                                                                        |

### <p style="text-align:center;">Git Commit (Updated Commands)</p>

| Commands                          | Description                                                                                                                         |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| git commit -m “feat: message”     | Create a new commit in a Git repository with a specific message to indicate a new feature commit in the repository.                 |
| git commit -m “fix: message”      | Create a new commit in a Git repository with a specific message to fix the bugs in codebases                                        |
| git commit -m “chore: message”    | Create a new commit in a Git repository with a specific message to show routine tasks or maintenance.                               |
| git commit -m “refactor: message” | Create a new commit in a Git repository with a specific message to change the code base and improve the structure.                  |
| git commit -m “docs: message”     | Create a new commit in a Git repository with a specific message to change the documentation.                                        |
| git commit -m “style: message”    | Create a new commit in a Git repository with a specific message to change the styling and formatting of the codebase.               |
| git commit -m “test: message”     | Create a new commit in a Git repository with a specific message to indicate test-related changes.                                   |
| git commit -m “perf: message”     | Create a new commit in a Git repository with a specific message to indicate performance-related changes.                            |
| git commit -m “ci: message”       | Create a new commit in a Git repository with a specific message to indicate the continuous integration (CI) system-related changes. |
| git commit -m “build: message”    | Create a new commit in a Git repository with a specific message to indicate the changes related to the build process.               |
| git commit -m “revert: message”   | Create a new commit in a Git repository with a specific message to indicate the changes related to revert a previous commit.        |

### <p style="text-align:center;">Branching and Merging</p>

| Commands                                         | Description                                                                                                                          |
| ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| git branch                                       | Lists all branches in the repository.                                                                                                |
| git branch &lt;branch-name&gt;                   | Creates a new branch with the specified name.                                                                                        |
| git branch -d &lt;branch-name&gt;                | Deletes the specified branch.                                                                                                        |
| git branch -a                                    | Lists all local and remote branches.                                                                                                 |
| git branch -r                                    | Lists all remote branches.                                                                                                           |
| git checkout &lt;branch-name&gt;                 | Switches to the specified branch.                                                                                                    |
| git checkout -b &lt;new-branch-name&gt;          | Creates a new branch and switches to it.                                                                                             |
| git checkout — &lt;file&gt;                      | Discards changes made to the specified file and revert it to the version in the last commit.                                         |
| git merge &lt;branch&gt;                         | Merges the specified branch into the current branch.                                                                                 |
| git log                                          | Displays the commit history of the current branch.                                                                                   |
| git log &lt;branch-d                             | Displays the commit history of the specified branch.                                                                                 |
| git log –follow &lt;file&gt;                     | Displays the commit history of a file, including its renames.                                                                        |
| git log –all                                     | Displays the commit history of all branches.                                                                                         |
| git stash                                        | Stashes the changes in the working directory, allowing you to switch to a different branch or commit without committing the changes. |
| git stash list                                   | Lists all stashes in the repository.                                                                                                 |
| git stash pop                                    | Applies and removes the most recent stash from the stash list.                                                                       |
| git stash drop                                   | Removes the most recent stash from the stash list.                                                                                   |
| git tag                                          | Lists all tags in the repository.                                                                                                    |
| git tag &lt;tag-name&gt;                         | Creates a lightweight tag at the current commit.                                                                                     |
| git tag &lt;tag-name&gt; &lt;commit&gt;          | Creates a lightweight tag at the specified commit.                                                                                   |
| git tag -a &lt;tag-name&gt; -m “&lt;message&gt;” | Creates an annotated tag at the current commit with a custom message.                                                                |

### <p style="text-align:center;">Remote Repositories</p>

| Commands                                | Description                                                                                        |
| --------------------------------------- | -------------------------------------------------------------------------------------------------- |
| git fetch                               | Retrieves change from a remote repository, including new branches and commit.                      |
| git fetch &lt;remote&gt;                | Retrieves change from the specified remote repository.                                             |
| git fetch –prune                        | Removes any remote-tracking branches that no longer exist on the remote repository.                |
| git pull                                | Fetches changes from the remote repository and merges them into the current branch.                |
| git pull &lt;remote&gt;                 | Fetches changes from the specified remote repository and merges them into the current branch.      |
| git pull –rebase                        | Fetches changes from the remote repository and rebases the current branch onto the updated branch. |
| git push                                | Pushes local commits to the remote repository.                                                     |
| git push &lt;remote&gt;                 | Pushes local commits to the specified remote repository.                                           |
| git push &lt;remote&gt; &lt;branch&gt;  | Pushes local commits to the specified branch of the remote repository.                             |
| git push –all                           | Pushes all branches to the remote repository.                                                      |
| git remote                              | Lists all remote repositories.                                                                     |
| git remote add &lt;name&gt; &lt;url&gt; | Adds a new remote repository with the specified name and URL.                                      |

### <p style="text-align:center;">Git Comparison</p>

| Commands                | Description                                                       |
| ----------------------- | ----------------------------------------------------------------- |
| git show                | Shows the details of a specific commit, including its changes.    |
| git show &lt;commit&gt; | Shows the details of the specified commit, including its changes. |

### <p style="text-align:center;">Git Managing History</p>

| Commands                             | Description                                                                              |
| ------------------------------------ | ---------------------------------------------------------------------------------------- |
| git revert &lt;commit&gt;            | Creates a new commit that undoes the changes introduced by the specified commit.         |
| git revert –no-commit &lt;commit&gt; | Undoes the changes introduced by the specified commit, but does not create a new commit. |
| git rebase &lt;branch&gt;            | Reapplies commits on the current branch onto the tip of the specified branch.            |
