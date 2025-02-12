# Customize settings

<!-- toc -->

## Via JSON

**Open the command palette and find Preferences: Open keyboard shortcuts (JSON) and paste the following:**

Switch focus between terminal and editor.

<kbd>ctrl</kbd> + <kbd>`</kbd>  

```json
{
    "key":     "ctrl+`",
    "command": "workbench.action.terminal.focus"
},
{
    "key":     "ctrl+`",
    "command": "workbench.action.focusActiveEditorGroup",
    "when":    "terminalFocus"
}
```

Adjust font size on terminal  

<kbd>ctrl</kbd> + <kbd>+</kbd> / <kbd>ctrl</kbd> + <kbd>-</kbd>  

```json
    {
        "key": "ctrl+=",
        "command": "workbench.action.terminal.fontZoomIn",
        "when": "terminalFocus"
    },
    {
        "key": "ctrl+-",
        "command": "workbench.action.terminal.fontZoomOut",
        "when": "terminalFocus"
    }
```

## Via UI

**Open the command palette and find Preferences: Open Settings UI or press <kbd>Ctrl</kbd> + <kbd>,</kbd>**  

Paste "editor.find.autoFindInSelection" in search settings and choose 'multiline' to enable to search selected text.  
