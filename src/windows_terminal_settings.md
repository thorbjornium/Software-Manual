# Settings

Yes, Windows Terminal has a "mark mode" that allows you to select text using the keyboard.

Here's how to use it:

1. **Enable Mark Mode:** Press `Ctrl + Shift + M`. This puts the terminal into mark mode.
2. **Select Text:** Use the arrow keys (with `Shift` held down) to move the cursor and select the desired text. You can also use `Ctrl + A` to select all text in the terminal buffer.
3. **Copy Text:** Once the text is selected, press `Ctrl + Shift + C` to copy it to the clipboard.

Some additional information about text selection in Windows Terminal:

- **Hyperlink Navigation:** In mark mode, you can use `Tab` or `Shift + Tab` to navigate between hyperlinks in the buffer if hyperlink detection is enabled (`experimental.detectUrls`).
- **Block Selection:** Use the `toggleBlockSelection` action to switch to block selection mode.
- **Copy on Select:** You can enable the `copyOnSelect` setting so that text is automatically copied to the clipboard when you select it. If this setting is enabled, right-clicking the terminal will paste the selected text.

{
"key": "ctrl+m",
"command": "editor.action.toggleTabFocusMode"
}

C:\Users\jense\AppData\Roaming\Microsoft\Windows\PowerShell\PSReadLine

Edit ConsoleHost_history.txt

**What is PSReadLine?**

`PSReadLine` is a module that replaces the default PowerShell console line editor with a more feature-rich and user-friendly one. It significantly improves the command-line experience by providing:

- **Command History:** Persistent and easily accessible command history.
- **Syntax Highlighting:** Makes code easier to read and identify errors.
- **Tab Completion:** Intelligent tab completion for commands, parameters, file paths, and more.
- **Vi Mode:** An optional editing mode that emulates the Vi/Vim text editor.
- **Predictive IntelliSense:** Suggests commands based on your history and other factors.
- **Key Bindings:** Customizable key bindings for common actions.

**PSReadLine History Features**

The history features in `PSReadLine` are a major benefit. Here's a breakdown:

- **Persistent History:** `PSReadLine` saves your command history to a file (by default, `~\AppData\Roaming\Microsoft\Windows\PowerShell\PSReadLine\ConsoleHost_history.txt`). This means your commands are remembered across PowerShell sessions.
- **History Navigation:**
  - **Up/Down Arrow Keys:** Navigate through your command history.
  - **Ctrl+R (Reverse Search):** Start typing a command, and `PSReadLine` will search backward through your history for matching commands. Press `Ctrl+R` repeatedly to cycle through matches.
  - **Ctrl+S (Forward Search):** Similar to `Ctrl+R`, but searches forward. (Less commonly used).
  - **`Get-History` Cmdlet:** Retrieves the command history from the current session. This is useful for scripting or further analysis.
  - **`Invoke-History` Cmdlet:** Executes a command from the history by its ID (as shown by `Get-History`).
- **History Duplicates:** By default, `PSReadLine` prevents consecutive duplicate commands from being added to the history.
- **History Size:** You can control the maximum number of commands saved in the history file and the maximum number of commands stored in the current session's history.
- **History Location:** You can change the location of the history file.

**Key Cmdlets for Managing History**

- **`Get-History`:**Retrieves the command history.
  - `Get-History`: Gets the history of the current session.
  - `Get-History -Id <ID>`: Gets a specific history entry by its ID.
  - `Get-History -Count <N>`: Gets the last N history entries.
- **`Invoke-History`:**Executes a command from the history.
  - `Invoke-History <ID>`: Executes the history entry with the specified ID. You can get the ID from `Get-History`.
- **`Add-History`:** Adds a command to the history. (Less commonly used directly).
- **`Clear-History`:**Clears the command history. Be careful with this one!
  - `Clear-History`: Clears the history of the current session.
  - `Clear-History -Newest <N>`: Deletes the newest N entries.
  - `Clear-History -Oldest <N>`: Deletes the oldest N entries.

**Configuration and Customization**

You can customize `PSReadLine`'s behavior using the `Set-PSReadLineOption` cmdlet. Here are some common settings related to history:

- **`HistorySavePath`:** Specifies the path to the history file.

```powershell
Set-PSReadLineOption -HistorySavePath "C:\MyPowerShellHistory.txt"
```

- **`MaximumHistoryCount`:** Specifies the maximum number of commands to store in the _session_ history.

```powershell
Set-PSReadLineOption -MaximumHistoryCount 4096
```

- **`MaximumKillRingCount`:** Specifies the maximum number of commands to store in the "kill ring" (used for cutting and pasting). This is less directly related to history but can affect your command editing experience.

```powershell
Set-PSReadLineOption -MaximumKillRingCount 10
```

- **`HistoryNoDuplicates`:** Specifies whether to prevent consecutive duplicate commands from being added to the history. (Defaults to `$true`).
- **`HistorySearchCaseSensitive`:** Specifies whether history searches (`Ctrl+R`, `Ctrl+S`) are case-sensitive. (Defaults to `$false`).
- **`HistorySaveStyle`:** Specifies how the history is saved. The default is `SaveIncrementally`, which saves each command to the history file as it's executed.

**Example: Configuring PSReadLine History**

To configure `PSReadLine` history, you typically add the `Set-PSReadLineOption` commands to your PowerShell profile. The profile is a script that runs automatically when PowerShell starts.

1. **Find Your Profile:**

```powershell
$PROFILE
```

    This will show you the path to your current PowerShell profile. If the file doesn't exist, you can create it.

2. **Edit Your Profile:** Open the profile file in a text editor (like VS Code or Notepad).
3. **Add Configuration:** Add the `Set-PSReadLineOption` commands to the profile. For example:

```powershell
Set-PSReadLineOption -HistorySavePath "$env:USERPROFILE\.powershell\MyHistory.txt"Set-PSReadLineOption -MaximumHistoryCount 10000Set-PSReadLineOption -HistoryNoDuplicates $true
```

4. **Save and Restart:** Save the profile file and restart PowerShell.

**Troubleshooting**

- **History Not Saving:**
  - Make sure `PSReadLine` is installed (`Get-Module PSReadLine -ListAvailable`). If not, install it: `Install-Module PSReadLine -Force`.
  - Check that the `HistorySavePath` is valid and that PowerShell has permissions to write to that location.
  - Ensure that `HistorySaveStyle` is set to `SaveIncrementally` (the default) or `SaveAtExit`.
- **History Not Showing Up:**
  - Verify that `MaximumHistoryCount` is set to a reasonable value.
  - Make sure you're using the correct key bindings for history navigation (Up/Down arrows, Ctrl+R, Ctrl+S).
  - If you've recently cleared the history, it will be empty.

**In summary:**

`PSReadLine` is essential for a productive PowerShell experience. Its history features provide persistent storage, easy navigation, and customization options to tailor the command-line to your needs. By understanding the key cmdlets and configuration settings, you can effectively manage your PowerShell command history and improve your workflow.
