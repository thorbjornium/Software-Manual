# VS Code

## Keyboard shortcuts  

<!-- toc -->

### Toggle terminal and editor focus  

Open Command Palette and find the keyboard shortcuts file (JSON) and paste the following:

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

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_basic-editing" target="_blank">Basic editing</a>  

| Command                                     | Key              |  
| :-------------------------------------------| :----------------|  
| Cut line (empty selection)                  | Ctrl+X           |  
| Copy line (empty selection)                 | Ctrl+C           |  
| Paste                                       | Ctrl+V           |  
| Delete Line                                 | Ctrl+Shift+K     |  
| Insert Line Below                           | Ctrl+Enter       |  
| Insert Line Above                           | Ctrl+Shift+Enter |  
| Move Line Down                              | Alt+Down         |  
| Move Line Up                                | Alt+Up           |  
| Copy Line Down                              | Shift+Alt+Down   |  
| Copy Line Up                                | Shift+Alt+Up     |  
| Undo                                        | Ctrl+Z           |  
| Redo                                        | Ctrl+Y           |  
| Add Selection To Next Find Match            | Ctrl+D           |  
| Move Last Selection To Next Find Match      | Ctrl+K Ctrl+D    |  
| Undo last cursor operation                  | Ctrl+U           |  
| Insert cursor at end of each line selected  | Shift+Alt+I      |  
| Select all occurrences of current selection | Ctrl+Shift+L     |  
| Select all occurrences of current word      | Ctrl+F2          |  
| Select current line                         | Ctrl+L           |  
| Insert Cursor Below                         | Ctrl+Alt+Down    |  
| Insert Cursor Above                         | Ctrl+Alt+Up      |  
| Jump to matching bracket                    | Ctrl+Shift+\     |  
| Indent Line                                 | Ctrl+]           |  
| Outdent Line                                | Ctrl+[           |  
| Go to Beginning of Line                     | Home             |  
| Go to End of Line                           | End              |  
| Go to End of File                           | Ctrl+End         |  
| Go to Beginning of File                     | Ctrl+Home        |  
| Scroll Line Down                            | Ctrl+Down        |  
| Scroll Line Up                              | Ctrl+Up          |  
| Scroll Page Down                            | Alt+PageDown     |  
| Scroll Page Up                              | Alt+PageUp       |  
| Fold (collapse) region                      | Ctrl+Shift+[     |  
| Unfold (uncollapse) region                  | Ctrl+Shift+]     |  
| Toggle Fold region                          | Ctrl+K Ctrl+L    |  
| Fold (collapse) all subregions              | Ctrl+K Ctrl+[    |  
| Unfold (uncollapse) all subregions          | Ctrl+K Ctrl+]    |  
| Fold (collapse) all regions                 | Ctrl+K Ctrl+0    |  
| Unfold (uncollapse) all regions             | Ctrl+K Ctrl+J    |  
| Add Line Comment                            | Ctrl+K Ctrl+C    |  
| Remove Line Comment                         | Ctrl+K Ctrl+U    |  
| Toggle Line Comment                         | Ctrl+/           |  
| Toggle Block Comment                        | Shift+Alt+A      |  
| Find                                        | Ctrl+F           |  
| Replace                                     | Ctrl+H           |  
| Find Next                                   | Enter            |  
| Find Previous                               | Shift+Enter      |  
| Select All Occurrences of Find Match        | Alt+Enter        |  
| Toggle Find Case Sensitive                  | Alt+C            |  
| Toggle Find Regex                           | Alt+R            |  
| Toggle Find Whole Word                      | Alt+W            |  
| Toggle Use of Tab Key for Setting Focus     | Ctrl+M           |  
| Toggle Render Whitespace                    | unassigned       |  
| Toggle Word Wrap                            | Alt+Z            |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_rich-languages-editing" target="_blank">Rich languages editing</a>  

| Command                     | Key              |  
| --------------------------- | ---------------- |  
| Trigger Suggest             | Ctrl+Space       |  
| Trigger Parameter Hints     | Ctrl+Shift+Space |  
| Format Document             | Shift+Alt+F      |  
| Format Selection            | Ctrl+K Ctrl+F    |  
| Go to Definition            | F12              |  
| Show Hover                  | Ctrl+K Ctrl+I    |  
| Peek Definition             | Alt+F12          |  
| Open Definition to the Side | Ctrl+K F12       |  
| Quick Fix                   | Ctrl+.           |  
| Go to References            | Shift+F12        |  
| Rename Symbol               | F2               |  
| Replace with Next Value     | Ctrl+Shift+.     |  
| Replace with Previous Value | Ctrl+Shift+,     |  
| Expand AST Selection        | Shift+Alt+Right  |  
| Shrink AST Selection        | Shift+Alt+Left   |  
| Trim Trailing Whitespace    | Ctrl+K Ctrl+X    |  
| Change Language Mode        | Ctrl+K M         |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_navigation" target="_blank">Navigation</a>  

| Command                         | Key                |  
| ------------------------------- | ------------------ |  
| Show All Symbols                | Ctrl+T             |  
| Go to Line...                   | Ctrl+G             |  
| Go to File..., Quick Open       | Ctrl+P             |  
| Go to Symbol...                 | Ctrl+Shift+O       |  
| Show Problems                   | Ctrl+Shift+M       |  
| Go to Next Error or Warning     | F8                 |  
| Go to Previous Error or Warning | Shift+F8           |  
| Show All Commands               | Ctrl+Shift+P or F1 |  
| Navigate Editor Group History   | Ctrl+Tab           |  
| Go Back                         | Alt+Left           |  
| Go back in Quick Input          | Alt+Left           |  
| Go Forward                      | Alt+Right          |  
| Focus Breadcrumbs               | Ctrl+Shift+;       |  
| Focus and Select Breadcrumbs    | Ctrl+Shift+.       |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_editorwindow-management" target="_blank">Editor/Window Management</a>  

| Command                              | Key                 |  
| ------------------------------------ | ------------------- |  
| New Window                           | Ctrl+Shift+N        |  
| Close Window                         | Alt+F4              |  
| Close Editor                         | Ctrl+F4             |  
| Close Folder                         | Ctrl+K F            |  
| Cycle Between Editor Groups          | unassigned          |  
| Split Editor                         | Ctrl+\              |  
| Focus into First Editor Group        | Ctrl+1              |  
| Focus into Second Editor Group       | Ctrl+2              |  
| Focus into Third Editor Group        | Ctrl+3              |  
| Focus into Editor Group on the Left  | unassigned          |  
| Focus into Editor Group on the Right | unassigned          |  
| Move Editor Left                     | Ctrl+Shift+PageUp   |  
| Move Editor Right                    | Ctrl+Shift+PageDown |  
| Move Active Editor Group Left        | Ctrl+K Left         |  
| Move Active Editor Group Right       | Ctrl+K Right        |  
| Move Editor into Next Group          | Ctrl+Alt+Right      |  
| Move Editor into Previous Group      | Ctrl+Alt+Left       |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_file-management" target="_blank">File Management</a>  

| Command                        | Key           |  
| ------------------------------ | ------------- |  
| New File                       | Ctrl+N        |  
| Open File...                   | Ctrl+O        |  
| Save                           | Ctrl+S        |  
| Save All                       | Ctrl+K S      |  
| Save As...                     | Ctrl+Shift+S  |  
| Close                          | Ctrl+F4       |  
| Close Others                   | unassigned    |  
| Close Group                    | Ctrl+K W      |  
| Close Other Groups             | unassigned    |  
| Close Group to Left            | unassigned    |  
| Close Group to Right           | unassigned    |  
| Close All                      | Ctrl+K Ctrl+W |  
| Reopen Closed Editor           | Ctrl+Shift+T  |  
| Keep Open                      | Ctrl+K Enter  |  
| Copy Path of Active File       | Ctrl+K P      |  
| Reveal Active File in Windows  | Ctrl+K R      |  
| Show Opened File in New Window | unassigned    |  
| Compare Opened File With       | unassigned    |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_display" target="_blank">Display</a>

| Command                      | Key           |  
| ---------------------------- | ------------- |  
| Toggle Full Screen           | F11           |  
| Toggle Zen Mode              | Ctrl+K Z      |  
| Leave Zen Mode               | Escape Escape |  
| Zoom in                      | Ctrl+=        |  
| Zoom out                     | Ctrl+-        |  
| Reset Zoom                   | Ctrl+Numpad0  |  
| Toggle Sidebar Visibility    | Ctrl+B        |  
| Show Explorer / Toggle Focus | Ctrl+Shift+E  |  
| Show Search                  | Ctrl+Shift+F  |  
| Show Source Control          | Ctrl+Shift+G  |  
| Show Run                     | Ctrl+Shift+D  |  
| Show Extensions              | Ctrl+Shift+X  |  
| Show Output                  | Ctrl+Shift+U  |  
| Quick Open View              | Ctrl+Q        |  
| Open New Command Prompt      | Ctrl+Shift+C  |  
| Toggle Markdown Preview      | Ctrl+Shift+V  |  
| Open Preview to the Side     | Ctrl+K V      |  
| Toggle Integrated Terminal   | Ctrl+`        |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_search" target="_blank">Search</a>  

| Command                       | Key          |  
| ----------------------------- | ------------ |  
| Show Search                   | Ctrl+Shift+F |  
| Replace in Files              | Ctrl+Shift+H |  
| Toggle Match Case             | Alt+C        |  
| Toggle Match Whole Word       | Alt+W        |  
| Toggle Use Regular Expression | Alt+R        |  
| Toggle Search Details         | Ctrl+Shift+J |  
| Focus Next Search Result      | F4           |  
| Focus Previous Search Result  | Shift+F4     |  
| Show Next Search Term         | Down         |  
| Show Previous Search Term     | Up           |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_search-editor" target="_blank">Search Editor</a>  

| Command                   | Key                  |  
| ------------------------- | -------------------- |  
| Open Results In Editor    | Alt+Enter            |  
| Focus Search Editor Input | Escape               |  
| Search Again              | Ctrl+Shift+R         |  
| Delete File Results       | Ctrl+Shift+Backspace |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_preferences" target="_blank">Preferences</a>  

| Command                    | Key           |  
| -------------------------- | ------------- |  
| Open Settings              | Ctrl+,        |  
| Open Workspace Settings    | unassigned    |  
| Open Keyboard Shortcuts    | Ctrl+K Ctrl+S |  
| Open User Snippets         | unassigned    |  
| Select Color Theme         | Ctrl+K Ctrl+T |  
| Configure Display Language | unassigned    |  

### <p align="center"><a href="https://code.visualstudio.com/docs/getstarted/keybindings#_debug" target="_blank">Debug</a>  

| Command                   | Key     |  
| ------------------------- | ------- |  
| Toggle Breakpoint         | F9      |  
| Start                     | F5      |  
| Continue                  | F5      |  
| Start (without debugging) | Ctrl+F5 |  
| Pause                     | F6      |  
| Step Into                 | F11     |  

### <p align="center"><a href="(https://code.visualstudio.com/docs/getstarted/keybindings#_tasks)" target="_blank">Tasks</a>  

| Command        | Key          |  
| -------------- | ------------ |  
| Run Build Task | Ctrl+Shift+B |  
| Run Test Task  | unassigned   |  

### <p align="center"><a href="(https://code.visualstudio.com/docs/getstarted/keybindings#_extensions)" target="_blank">Extensions</a>  

| Command                     | Key        |  
| --------------------------- | ---------- |  
| Install Extension           | unassigned |  
| Show Installed Extensions   | unassigned |  
| Show Outdated Extensions    | unassigned |  
| Show Recommended Extensions | unassigned |  
| Show Popular Extensions     | unassigned |  
| Update All Extensions       | unassigned |  
