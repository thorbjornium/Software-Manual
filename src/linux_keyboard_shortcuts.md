# Keyboard Shortcuts

<!-- toc -->

## I. General Desktop Environment Shortcuts

These shortcuts often use the **Super key** (usually the Windows key on most keyboards) or a combination of <kbd>Ctrl</kbd> + <kbd>Alt</kbd>.

### Window Management

* <kbd>Alt</kbd> + <kbd>Tab</kbd>: Switch between open applications/windows (hold <kbd>Alt</kbd> and repeatedly press <kbd>Tab</kbd> to cycle; release <kbd>Alt</kbd> to select). Use <kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>Tab</kbd> for reverse order.
* <kbd>Super</kbd> + <kbd>Tab</kbd>: (GNOME) Similar to <kbd>Alt</kbd> + <kbd>Tab</kbd>, often integrated with Activities overview.
* <kbd>Super</kbd> + <kbd>D</kbd> or <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>D</kbd>: Show/Hide Desktop (minimize/restore all windows).
* <kbd>Alt</kbd> + <kbd>F4</kbd>: Close the current application window.
* <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Esc</kbd>: Force quit an unresponsive application (changes cursor to a "kill" icon).
* <kbd>Super</kbd> + <kbd>Left Arrow</kbd>: Snap window to left half of the screen (tiling).
* <kbd>Super</kbd> + <kbd>Right Arrow</kbd>: Snap window to right half of the screen (tiling).
* <kbd>Super</kbd> + <kbd>Up Arrow</kbd>: Maximize window.
* <kbd>Super</kbd> + <kbd>Down Arrow</kbd>: Restore/Minimize window.

### Workspace Management (Virtual Desktops)

* <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Left Arrow</kbd>: Switch to the left workspace.
* <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Right Arrow</kbd>: Switch to the right workspace.
* <kbd>Shift</kbd> + <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Left Arrow</kbd>: Move the current window to the left workspace.
* <kbd>Shift</kbd> + <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Right Arrow</kbd>: Move the current window to the right workspace.
* <kbd>Super</kbd> + <kbd>Page Down</kbd>: (GNOME) Switch to the next workspace.
* <kbd>Super</kbd> + <kbd>Page Up</kbd>: (GNOME) Switch to the previous workspace.
* <kbd>Shift</kbd> + <kbd>Super</kbd> + <kbd>Page Down</kbd>: (GNOME) Move current window to the next workspace.
* <kbd>Shift</kbd> + <kbd>Super</kbd> + <kbd>Page Up</kbd>: (GNOME) Move current window to the previous workspace.

### System Actions

* <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>T</kbd>: Open a new terminal window.
* <kbd>Alt</kbd> + <kbd>F2</kbd>: Open a "Run Command" dialog (e.g., KRunner in KDE).
* <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>L</kbd> or <kbd>Super</kbd> + <kbd>L</kbd>: Lock the screen.
* <kbd>Print Screen</kbd> (PrtSc): Take a screenshot of the entire screen.
* <kbd>Alt</kbd> + <kbd>Print Screen</kbd>: Take a screenshot of the active window.
* <kbd>Shift</kbd> + <kbd>Print Screen</kbd>: Take a screenshot of a selected area.
* <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Delete</kbd>: Bring up shutdown/logout options.
* <kbd>Super</kbd> key (just press it): Opens the Activities overview, application launcher, or main menu, depending on your DE (e.g., GNOME Activities, KDE Plasma's Application Launcher).

### Universal Text Editing (GUI Applications)

* <kbd>Ctrl</kbd> + <kbd>C</kbd>: Copy selected text.
* <kbd>Ctrl</kbd> + <kbd>X</kbd>: Cut selected text.
* <kbd>Ctrl</kbd> + <kbd>V</kbd>: Paste text.
* <kbd>Ctrl</kbd> + <kbd>Z</kbd>: Undo the last action.
* <kbd>Ctrl</kbd> + <kbd>Y</kbd> or <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Z</kbd>: Redo the last undone action.
* <kbd>Ctrl</kbd> + <kbd>A</kbd>: Select all.
* <kbd>Ctrl</kbd> + <kbd>S</kbd>: Save.
* <kbd>Ctrl</kbd> + <kbd>N</kbd>: New document/file/window.
* <kbd>Ctrl</kbd> + <kbd>O</kbd>: Open file.
* <kbd>Ctrl</kbd> + <kbd>P</kbd>: Print.
* <kbd>Ctrl</kbd> + <kbd>F</kbd>: Find (in current document/application).
* <kbd>F1</kbd>: Open help.

---

## II. Terminal (Bash/Zsh/Shell) Shortcuts

These shortcuts are primarily for navigating and editing text within the command line itself.

### Cursor Movement

* <kbd>Ctrl</kbd> + <kbd>A</kbd> (or <kbd>Home</kbd>): Move cursor to the beginning of the line.
* <kbd>Ctrl</kbd> + <kbd>E</kbd> (or <kbd>End</kbd>): Move cursor to the end of the line.
* <kbd>Ctrl</kbd> + <kbd>B</kbd> (or <kbd>Left Arrow</kbd>): Move cursor backward one character.
* <kbd>Ctrl</kbd> + <kbd>F</kbd> (or <kbd>Right Arrow</kbd>): Move cursor forward one character.
* <kbd>Alt</kbd> + <kbd>B</kbd> (or <kbd>Ctrl</kbd> + <kbd>Left Arrow</kbd>): Move cursor backward one word.
* <kbd>Alt</kbd> + <kbd>F</kbd> (or <kbd>Ctrl</kbd> + <kbd>Right Arrow</kbd>): Move cursor forward one word.
* <kbd>Ctrl</kbd> + <kbd>X</kbd> (tap twice quickly): Toggle between the start of the line and the current cursor position.

### Text Editing/Deletion

* <kbd>Ctrl</kbd> + <kbd>H</kbd> (or <kbd>Backspace</kbd>): Delete character before cursor.
* <kbd>Ctrl</kbd> + <kbd>D</kbd> (or <kbd>Delete</kbd>): Delete character under cursor.
* <kbd>Alt</kbd> + <kbd>Backspace</kbd> or <kbd>Ctrl</kbd> + <kbd>W</kbd>: Delete the word before the cursor.
* <kbd>Alt</kbd> + <kbd>D</kbd>: Delete the word from the cursor to the end of the word.
* <kbd>Ctrl</kbd> + <kbd>U</kbd>: Delete (cut) everything from the cursor to the beginning of the line.
* <kbd>Ctrl</kbd> + <kbd>K</kbd>: Delete (cut) everything from the cursor to the end of the line.
* <kbd>Ctrl</kbd> + <kbd>Y</kbd>: Paste the last deleted/cut text.
* <kbd>Ctrl</kbd> + <kbd>T</kbd>: Transpose (swap) the last two characters before the cursor.
* <kbd>Alt</kbd> + <kbd>T</kbd>: Transpose (swap) the last two words before the cursor.
* <kbd>Alt</kbd> + <kbd>U</kbd>: Uppercase from cursor to end of word.
* <kbd>Alt</kbd> + <kbd>L</kbd>: Lowercase from cursor to end of word.
* <kbd>Alt</kbd> + <kbd>C</kbd>: Capitalize the first character of the word.

### History Navigation & Search

* <kbd>Up Arrow</kbd> / <kbd>Down Arrow</kbd>: Scroll through previous/next commands in history.
* <kbd>Ctrl</kbd> + <kbd>R</kbd>: Reverse incremental search through command history (type to search, <kbd>Ctrl</kbd> + <kbd>R</kbd> again for next match, <kbd>Enter</kbd> to execute, <kbd>Esc</kbd> to exit).
* <kbd>Ctrl</kbd> + <kbd>G</kbd>: Exit history search mode.
* <kbd>!!</kbd>: Execute the last command.
* <kbd>!string</kbd>: Execute the last command starting with "string".
* <kbd>!$</kbd>: Use the last argument of the previous command.

### Terminal Control

* <kbd>Tab</kbd>: Autocomplete commands, filenames, or directory names (press twice for options if ambiguous).
* <kbd>Ctrl</kbd> + <kbd>C</kbd>: Interrupt/terminate the currently running foreground process.
* <kbd>Ctrl</kbd> + <kbd>D</kbd>: Send an End-of-File (EOF) marker (can log you out or exit a program).
* <kbd>Ctrl</kbd> + <kbd>Z</kbd>: Suspend the current foreground process.
* <kbd>Ctrl</kbd> + <kbd>L</kbd>: Clear the terminal screen.
* <kbd>Ctrl</kbd> + <kbd>S</kbd>: Pause terminal output.
* <kbd>Ctrl</kbd> + <kbd>Q</kbd>: Resume terminal output.
* <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>C</kbd>: Copy selected text from the terminal.
* <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>V</kbd>: Paste text into the terminal.
* <kbd>Shift</kbd> + <kbd>Insert</kbd>: Alternative paste (often uses the primary selection buffer).

---

## III. System/Boot Level Shortcuts

These shortcuts are less common for daily use but are useful for troubleshooting or direct system interaction.

* <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>F1</kbd> to <kbd>F6</kbd>: Switch to a virtual console (text-based login screen). Use <kbd>F7</kbd> or <kbd>F8</kbd> to return to the graphical desktop.
* <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>Backspace</kbd>: Kills the X server, returning to the login screen (often disabled by default).
* **Magic SysRq Key:** A powerful kernel-level debugging tool for emergencies (requires <kbd>SysRq</kbd> key, often shared with <kbd>Print Screen</kbd>). **Use with extreme caution!**
  * <kbd>Alt</kbd> + <kbd>SysRq</kbd> + <kbd>R</kbd>: Raw (take keyboard control).
  * <kbd>Alt</kbd> + <kbd>SysRq</kbd> + <kbd>E</kbd>: Terminate (send SIGTERM to processes).
  * <kbd>Alt</kbd> + <kbd>SysRq</kbd> + <kbd>I</kbd>: Kill (send SIGKILL to processes).
  * <kbd>Alt</kbd> + <kbd>SysRq</kbd> + <kbd>S</kbd>: Sync (sync disk).
  * <kbd>Alt</kbd> + <kbd>SysRq</kbd> + <kbd>U</kbd>: Unmount (remount all filesystems read-only).
  * <kbd>Alt</kbd> + <kbd>SysRq</kbd> + <kbd>B</kbd>: Reboot (immediate reboot).
  * Common mnemonic for a safe reboot: <kbd>R</kbd> <kbd>E</kbd> <kbd>I</kbd> <kbd>S</kbd> <kbd>U</kbd> <kbd>B</kbd> (Raising Elephants Is So Utterly Boring)
