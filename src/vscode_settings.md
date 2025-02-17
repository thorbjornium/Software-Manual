# Customize settings

<!-- toc -->

## Keyboard Settings Via JSON

**Open the command palette and find _Preferences: Open keyboard shortcuts (JSON)_ and paste the following:**

### Switch focus between terminal and editor

<kbd>Ctrl</kbd> + <kbd>`</kbd>

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

### Adjust font size on terminal

<kbd>Ctrl</kbd> + <kbd>+</kbd> / <kbd>Ctrl</kbd> + <kbd>-</kbd>

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

### New file

<kbd>Ctrl</kbd> + <kbd>n</kbd>

```json
    {
        "key": "ctrl+n",
        "command": "workbench.action.files.newUntitledFile"
    },
    {
        "key": "ctrl+n",
        "command": "-workbench.action.files.newUntitledFile"
    },
```

### Focus activity bar

The most far left sidebar is the activity bar.

<kbd>Ctrl</kbd> + <kbd>k</kbd> + <kbd>a</kbd>

```json
    {
        "key": "ctrl+k a",
        "command": "workbench.action.focusActivityBar"
    },
```

### Inline suggestions: acceptNextWord

<kbd>Ctrl</kbd> + <kbd>→</kbd>

```json
    {
        "key": "ctrl+right",
        "command": "editor.action.inlineSuggest.acceptNextWord",
        "when": "inlineSuggestionVisible && !editorReadonly"
    },
```
## Settings Via JSON

**Open the command palette and find *Preferences: Open User Settings (JSON)* and paste the following:**

### Errorlens

<details>

<summary>Code</summary>

```json
"errorLens.codeLensTemplate": "$severity $message $source",
  "errorLens.statusBarMessageEnabled": true,
  "errorLens.statusBarColorsEnabled": true,
  "errorLens.statusBarMessageTemplate": "$message $severity $source",
  "errorLens.decorations": {
    "errorMessage": {
      "textDecoration": ";background:linear-gradient(to right, #0088ff, #0a9c33);border-radius:0.3em;padding:0 0.5ch;",
      "color": "#fff",
      // "fontWeight": "bold",
    },
    "warningMessage": {
      "textDecoration": ";background:linear-gradient(to right, #f73a00, #65d701);border-radius:0.3em;padding:0 0.5ch;",
      "color": "#fff",
      // "fontWeight": "bold",
    },
  },
  "errorLens.excludeBySource": [
    "markdownlint(MD033)"
  ],
  "errorLens.enabledDiagnosticLevels": [
    "error",
    "warning",
    "info",
    "hint"
  ],

```
</details>

## Settings Via UI

**Open the command palette and find _Preferences: Open Settings UI_ or press <kbd>Ctrl</kbd> + <kbd>,</kbd>**

### Search in selection

Paste "editor.find.autoFindInSelection" in search settings and choose 'multiline' to enable to search selection.
