# VS Code

## Keyboard settings  

### Toggle terminal and editor focus  

Open Command Palette and find keyboard shortcuts (JSON)

```  
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

### Adjust font size on terminal  

```
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
