# Basic editing functions

<!-- toc -->

| Key              | Function                      | Description                                                                                                                                                                                        |
| :--------------- | :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Unbound          | Abort                         | Abort the current operation, e.g. incremental history search                                                                                                                                       |
| Unbound          | AcceptAndGetNext              | Accept the current line and recall the next line from history after the current line finishes executing                                                                                            |
| Enter            | AcceptLine                    | Accept the input or move to the next line if input is missing a closing token.                                                                                                                     |
| Shift+Enter      | AddLine                       | Move the cursor to the next line without attempting to execute the input                                                                                                                           |
| Backspace        | BackwardDeleteChar            | Delete the character before the cursor                                                                                                                                                             |
| Ctrl+h           | BackwardDeleteChar            | Delete the character before the cursor                                                                                                                                                             |
| Ctrl+Home        | BackwardDeleteInput           | Delete text from the cursor to the start of the input                                                                                                                                              |
| Unbound          | BackwardDeleteLine            | Delete text from the cursor to the start of the current logical line                                                                                                                               |
| Unbound          | BackwardDeleteWord            | Delete the previous word in the line.                                                                                                                                                              |
| Unbound          | BackwardKillInput             | Move the text from the cursor to the beginning of the input to the kill ring                                                                                                                       |
| Unbound          | BackwardKillLine              | Move the text from the start of the current logical line to the cursor to the kill ring                                                                                                            |
| Ctrl+Backspace   | BackwardKillWord              | Move the text from the start of the current or previous word to the cursor to the kill ring                                                                                                        |
| Ctrl+w           | BackwardKillWord              | Move the text from the start of the current or previous word to the cursor to the kill ring                                                                                                        |
| Unbound          | CancelLine                    | Abort editing the current line and re-evaluate the prompt                                                                                                                                          |
| Unbound          | CapitalizeWord                | Find the next word starting from the current position and then upcase the first character and downcase the remaining characters.                                                                   |
| Ctrl+C           | Copy                          | Copy selected region to the system clipboard. If no region is selected, copy the whole line                                                                                                        |
| Ctrl+c           | CopyOrCancelLine              | Either copy selected text to the clipboard, or if no text is selected, cancel editing the line with CancelLine.                                                                                    |
| Ctrl+x           | Cut                           | Delete selected region placing deleted text in the system clipboard                                                                                                                                |
| Delete           | DeleteChar                    | Delete the character under the cursor                                                                                                                                                              |
| Unbound          | DeleteCharOrExit              | Delete the character under the cursor, or if the line is empty, exit the process.                                                                                                                  |
| Unbound          | DeleteEndOfBuffer             | Delete the current logical line and up to the end of the multiline buffer                                                                                                                          |
| Unbound          | DeleteEndOfWord               | Delete to the end of the current word, as delimited by white space and common delimiters.                                                                                                          |
| Unbound          | DeleteLine                    | Deletes the current line.                                                                                                                                                                          |
| Unbound          | DeleteLineToFirstChar         | Deletes from the first non blank character of the current logical line in a multiline buffer.                                                                                                      |
| Unbound          | DeletePreviousLines           | Deletes from the previous n logical lines in a multiline buffer to the current logical line included.                                                                                              |
| Unbound          | DeleteToEnd                   | Deletes from the cursor to the end of the line.                                                                                                                                                    |
| Unbound          | DeleteWord                    | Deletes the current word.                                                                                                                                                                          |
| Unbound          | DowncaseWord                  | Find the next word starting from the current position and then make it lower case.                                                                                                                 |
| Ctrl+End         | ForwardDeleteInput            | Delete text from the cursor to the end of the input                                                                                                                                                |
| Unbound          | ForwardDeleteLine             | Delete text from the cursor to the end of the current logical line                                                                                                                                 |
| Ctrl+Enter       | InsertLineAbove               | Inserts a new empty line above the current line without attempting to execute the input                                                                                                            |
| Shift+Ctrl+Enter | InsertLineBelow               | Inserts a new empty line below the current line without attempting to execute the input                                                                                                            |
| Unbound          | InvertCase                    | Inverts the case of the current character and advances the cursor.                                                                                                                                 |
| Unbound          | KillLine                      | Move the text from the cursor to the end of the input to the kill ring                                                                                                                             |
| Unbound          | KillRegion                    | Kill the text between the cursor and the mark                                                                                                                                                      |
| Alt+d            | KillWord                      | Move the text from the cursor to the end of the current or next word to the kill ring                                                                                                              |
| Ctrl+Delete      | KillWord                      | Move the text from the cursor to the end of the current or next word to the kill ring                                                                                                              |
| Ctrl+v           | Paste                         | Paste text from the system clipboard                                                                                                                                                               |
| Shift+Insert     | Paste                         | Paste text from the system clipboard                                                                                                                                                               |
| Unbound          | PasteAfter                    | Write the contents of the local clipboard after the cursor.                                                                                                                                        |
| Unbound          | PasteBefore                   | Write the contents of the local clipboard before the cursor.                                                                                                                                       |
| Unbound          | PrependAndAccept              | Inserts the entered character at the beginning and accepts the line.                                                                                                                               |
| Ctrl+y           | Redo                          | Redo an undo                                                                                                                                                                                       |
| Unbound          | RepeatLastCommand             | Repeats the last modification command.                                                                                                                                                             |
| Escape           | RevertLine                    | Equivalent to undo all edits (clears the line except lines imported from history)                                                                                                                  |
| Unbound          | ShellBackwardKillWord         | Move the text from the cursor to the start of the current or previous token to the kill ring                                                                                                       |
| Unbound          | ShellKillWord                 | Move the text from the cursor to the end of the current or next token to the kill ring                                                                                                             |
| Unbound          | SwapCharacters                | Swap the current character with the character before it.                                                                                                                                           |
| Ctrl+z           | Undo                          | Undo a previous edit                                                                                                                                                                               |
| Unbound          | UndoAll                       | Undoes all commands for this line.                                                                                                                                                                 |
| Unbound          | UnixWordRubout                | Move the text from the cursor to the start of the current or previous whitespace delimited word to the kill ring                                                                                   |
| Unbound          | UpcaseWord                    | Find the next word starting from the current position and then make it upper case.                                                                                                                 |
| Unbound          | ValidateAndAcceptLine         | Accept the input or move to the next line if input is missing a closing token. If there are other parse errors, unresolved commands, or incorrect parameters, show the error and continue editing. |
| Unbound          | ViAcceptLine                  | Accept the line and switch to Vi's insert mode.                                                                                                                                                    |
| Unbound          | ViAcceptLineOrExit            | If the line is empty, exit, otherwise accept the line as input.                                                                                                                                    |
| Unbound          | ViAppendLine                  | Appends a new multi-line edit mode line to the current line.                                                                                                                                       |
| Unbound          | ViBackwardDeleteGlob          | Delete backward to the beginning of the previous word, as delimited by white space.                                                                                                                |
| Unbound          | ViBackwardGlob                | Move the cursor to the beginning of the previous word, as delimited by white space.                                                                                                                |
| Unbound          | ViDeleteBrace                 | Deletes all characters between the cursor and the matching brace.                                                                                                                                  |
| Unbound          | ViDeleteEndOfGlob             | Delete to the end of this word, as delimited by white space.                                                                                                                                       |
| Unbound          | ViDeleteGlob                  | Delete the current word, as delimited by white space.                                                                                                                                              |
| Unbound          | ViDeleteToBeforeChar          | Deletes until given character.                                                                                                                                                                     |
| Unbound          | ViDeleteToBeforeCharBackward  | Deletes until given character.                                                                                                                                                                     |
| Unbound          | ViDeleteToChar                | Deletes until given character.                                                                                                                                                                     |
| Unbound          | ViDeleteToCharBackward        | Deletes backwards until given character.                                                                                                                                                           |
| Unbound          | ViInsertAtBegining            | Moves the cursor to the beginning of the line and switches to insert mode.                                                                                                                         |
| Unbound          | ViInsertAtEnd                 | Moves the cursor to the end of the line and switches to insert mode.                                                                                                                               |
| Unbound          | ViInsertLine                  | Inserts a new multi-line edit mode line in front of the current line.                                                                                                                              |
| Unbound          | ViInsertWithAppend            | Switch to insert mode, appending at the current line position.                                                                                                                                     |
| Unbound          | ViInsertWithDelete            | Deletes the current character and switches to insert mode.                                                                                                                                         |
| Unbound          | ViJoinLines                   | Joins the current multi-line edit mode line with the next.                                                                                                                                         |
| Unbound          | ViReplaceLine                 | Repace the current line with the next set of characters typed.                                                                                                                                     |
| Unbound          | ViReplaceToBeforeChar         | Replaces until given character.                                                                                                                                                                    |
| Unbound          | ViReplaceToBeforeCharBackward | Replaces until given character.                                                                                                                                                                    |
| Unbound          | ViReplaceToChar               | Deletes until given character.                                                                                                                                                                     |
| Unbound          | ViReplaceToCharBackward       | Replaces until given character.                                                                                                                                                                    |
| Unbound          | ViYankBeginningOfLine         | Place the characters before the cursor into the local clipboard.                                                                                                                                   |
| Unbound          | ViYankEndOfGlob               | Place the characters from the cursor to the end of the next white space delimited word into the local clipboard.                                                                                   |
| Unbound          | ViYankEndOfWord               | Place the characters from the cursor to the end of the next word, as delimited by white space and common delimiters, into the local clipboard.                                                     |
| Unbound          | ViYankLeft                    | Place the character to the left of the cursor into the local clipboard.                                                                                                                            |
| Unbound          | ViYankLine                    | Place all characters in the current line into the local clipboard.                                                                                                                                 |
| Unbound          | ViYankNextGlob                | Place all characters from the cursor to the end of the word, as delimited by white space, into the local clipboard.                                                                                |
| Unbound          | ViYankNextWord                | Place all characters from the cursor to the end of the word, as delimited by white space and common delimiters, into the local clipboard.                                                          |
| Unbound          | ViYankPercent                 | Place all characters between the matching brace and the cursor into the local clipboard.                                                                                                           |
| Unbound          | ViYankPreviousGlob            | Place all characters from before the cursor to the beginning of the previous word, as delimited by white space, into the local clipboard.                                                          |
| Unbound          | ViYankPreviousWord            | Place all characters from before the cursor to the beginning of the previous word, as delimited by white space and common delimiters, into the local clipboard.                                    |
| Unbound          | ViYankRight                   | Place the character at the cursor into the local clipboard.                                                                                                                                        |
| Unbound          | ViYankToEndOfLine             | Place all characters at and after the cursor into the local clipboard.                                                                                                                             |
| Unbound          | ViYankToFirstChar             | Place all characters before the cursor and to the 1st non-white space character into the local clipboard.                                                                                          |
| Unbound          | Yank                          | Copy the text from the current kill ring position to the input                                                                                                                                     |
| Alt+.            | YankLastArg                   | Copy the text of the last argument to the input                                                                                                                                                    |
| Unbound          | YankNthArg                    | Copy the text of the first argument to the input                                                                                                                                                   |
| Unbound          | YankPop                       | Replace the previously yanked text with the text from the next kill ring position                                                                                                                  |

## Cursor movement functions

| Key             | Function                | Description                                                                                                                      |
| :-------------- | :---------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| LeftArrow       | BackwardChar            | Move the cursor back one character                                                                                               |
| Ctrl+LeftArrow  | BackwardWord            | Move the cursor to the beginning of the current or previous word                                                                 |
| Home            | BeginningOfLine         | Move the cursor to the beginning of the line                                                                                     |
| End             | EndOfLine               | Move the cursor to the end of the line                                                                                           |
| RightArrow      | ForwardChar             | Move the cursor forward one character                                                                                            |
| Unbound         | ForwardWord             | Move the cursor forward to the end of the current word, or if between words, to the end of the next word.                        |
| Ctrl+]          | GotoBrace               | Go to matching brace                                                                                                             |
| Unbound         | GotoColumn              | Moves the cursor to the prescribed column.                                                                                       |
| Unbound         | GotoFirstNonBlankOfLine | Positions the cursor at the first non-blank character.                                                                           |
| Unbound         | MoveToEndOfLine         | Move to the end of the line.                                                                                                     |
| Unbound         | NextLine                | Move the cursor to the next line if the input has multiple lines.                                                                |
| Ctrl+RightArrow | NextWord                | Move the cursor forward to the start of the next word                                                                            |
| Unbound         | NextWordEnd             | Moves the cursor forward to the end of the next word.                                                                            |
| Unbound         | PreviousLine            | Move the cursor to the previous line if the input has multiple lines.                                                            |
| Unbound         | ShellBackwardWord       | Move the cursor to the beginning of the current or previous token or start of the line                                           |
| Unbound         | ShellForwardWord        | Move the cursor to the beginning of the next token or end of line                                                                |
| Unbound         | ShellNextWord           | Move the cursor to the end of the current token                                                                                  |
| Unbound         | ViBackwardChar          | Move the cursor back one character in the Vi edit mode.                                                                          |
| Unbound         | ViBackwardWord          | Delete backward to the beginning of the previous word, as delimited by white space and common delimiters, and enter insert mode. |
| Unbound         | ViEndOfGlob             | Move the cursor to the end this word, as delimited by white space.                                                               |
| Unbound         | ViEndOfPreviousGlob     | Moves to the end of the previous word, using only white space as a word delimiter.                                               |
| Unbound         | ViForwardChar           | Move the cursor forward one character in the Vi edit mode.                                                                       |
| Unbound         | ViGotoBrace             | Move the cursor to the matching brace.                                                                                           |
| Unbound         | ViNextGlob              | Move the cursor to the beginning of the next word, as delimited by white space.                                                  |
| Unbound         | ViNextWord              | Move the cursor to the beginning of the next word, as delimited by white space and common delimiters.                            |

## History functions

| Key       | Function                | Description                                                                                                                 |
| :-------- | :---------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| Unbound   | BeginningOfHistory      | Move to the first item in the history                                                                                       |
| Alt+F7    | ClearHistory            | Remove all items from the command line history (not PowerShell history)                                                     |
| Unbound   | EndOfHistory            | Move to the last item (the current input) in the history                                                                    |
| Ctrl+s    | ForwardSearchHistory    | Search history forward interactively                                                                                        |
| F8        | HistorySearchBackward   | Search for the previous item in the history that starts with the current input - like PreviousHistory if the input is empty |
| Shift+F8  | HistorySearchForward    | Search for the next item in the history that starts with the current input - like NextHistory if the input is empty         |
| DownArrow | NextHistory             | Replace the input with the next item in the history                                                                         |
| UpArrow   | PreviousHistory         | Replace the input with the previous item in the history                                                                     |
| Unbound   | ReverseSearchHistory    | Search history backwards interactively                                                                                      |
| Unbound   | ViSearchHistoryBackward | Starts a new search backward in the history.                                                                                |

## Completion functions

| Key       | Function              | Description                                                                                                                                                                  |
| :-------- | :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Unbound   | Complete              | Complete the input if there is a single completion, otherwise complete the input with common prefix for all completions. Show possible completions if pressed a second time. |
| Ctrl+@    | MenuComplete          | Complete the input if there is a single completion, otherwise complete the input by selecting from a menu of possible completions.                                           |
| Unbound   | PossibleCompletions   | Display the possible completions without changing the input                                                                                                                  |
| Unbound   | TabCompleteNext       | Complete the input using the next completion                                                                                                                                 |
| Shift+Tab | TabCompletePrevious   | Complete the input using the previous completion                                                                                                                             |
| Unbound   | ViTabCompleteNext     | Invokes TabCompleteNext after doing some vi-specific clean up.                                                                                                               |
| Unbound   | ViTabCompletePrevious | Invokes TabCompletePrevious after doing some vi-specific clean up.                                                                                                           |

## Prediction functions

| Key     | Function                  | Description                                                                                     |
| :------ | :------------------------ | :---------------------------------------------------------------------------------------------- |
| Ctrl+g  | AcceptNextSuggestionWord  | Accept the next word of the inline or selected suggestion                                       |
| Unbound | AcceptSuggestion          | Accept the current inline or selected suggestion                                                |
| Unbound | NextSuggestion            | Select the next suggestion item shown in the list view.                                         |
| Unbound | PreviousSuggestion        | Select the previous suggestion item shown in the list view.                                     |
| F4      | ShowFullPredictionTooltip | Show the full tooltip of the selected list-view item in the terminal's alternate screen buffer. |
| F2      | SwitchPredictionView      | Switch between the inline and list prediction views.                                            |

## Miscellaneous functions

| Key           | Function               | Description                                                                                                                                    |
| :------------ | :--------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Unbound       | CaptureScreen          | Allows you to select multiple lines from the console using Shift+UpArrow/DownArrow and copy the selected lines to clipboard by pressing Enter. |
| Ctrl+l        | ClearScreen            | Clear the screen and redraw the current line at the top of the screen                                                                          |
| Alt+0         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+1         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+2         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+3         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+4         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+5         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+6         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+7         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+8         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+9         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Alt+-         | DigitArgument          | Start or accumulate a numeric argument to other functions                                                                                      |
| Unbound       | InvokePrompt           | Erases the current prompt and calls the prompt function to redisplay the prompt                                                                |
| PageDown      | ScrollDisplayDown      | Scroll the display down one screen                                                                                                             |
| Ctrl+PageDown | ScrollDisplayDownLine  | Scroll the display down one line                                                                                                               |
| Unbound       | ScrollDisplayToCursor  | Scroll the display to the cursor                                                                                                               |
| Unbound       | ScrollDisplayTop       | Scroll the display to the top                                                                                                                  |
| PageUp        | ScrollDisplayUp        | Scroll the display up one screen                                                                                                               |
| Ctrl+PageUp   | ScrollDisplayUpLine    | Scroll the display up one line                                                                                                                 |
| F1            | ShowCommandHelp        | Shows help for the command at the cursor in an alternate screen buffer.                                                                        |
| Ctrl+Alt+?    | ShowKeyBindings        | Show all key bindings                                                                                                                          |
| Alt+h         | ShowParameterHelp      | Shows help for the parameter at the cursor.                                                                                                    |
| Unbound       | ViCommandMode          | Switch to VI's command mode.                                                                                                                   |
| Unbound       | ViDigitArgumentInChord | Handles the processing of a number argument after the first key of a chord.                                                                    |
| Unbound       | ViEditVisually         | Invokes the console compatible editor specified by $env:VISUAL or $env:EDITOR on the current command line.                                     |
| Unbound       | ViExit                 | Exit the shell.                                                                                                                                |
| Unbound       | ViInsertMode           | Switches to insert mode.                                                                                                                       |
| Alt+?         | WhatIsKey              | Show the key binding for the next chord entered                                                                                                |

## Selection functions

| Key                   | Function                | Description                                                                           |
| :-------------------- | :---------------------- | :------------------------------------------------------------------------------------ |
| Unbound               | ExchangePointAndMark    | Mark the location of the cursor and move the cursor to the position of the previous m |
|                       |                         | ark                                                                                   |
| Ctrl+a                | SelectAll               | Select the entire line. Moves the cursor to the end of the line                       |
| Shift+LeftArrow       | SelectBackwardChar      | Adjust the current selection to include the previous character                        |
| Shift+Home            | SelectBackwardsLine     | Adjust the current selection to include from the cursor to the start of the line      |
| Shift+Ctrl+LeftArrow  | SelectBackwardWord      | Adjust the current selection to include the previous word                             |
| Alt+a                 | SelectCommandArgument   | Make visual selection of the command arguments.                                       |
| Shift+RightArrow      | SelectForwardChar       | Adjust the current selection to include the next character                            |
| Unbound               | SelectForwardWord       | Adjust the current selection to include the next word using ForwardWord               |
| Shift+End             | SelectLine              | Adjust the current selection to include from the cursor to the end of the line        |
| Shift+Ctrl+RightArrow | SelectNextWord          | Adjust the current selection to include the next word                                 |
| Unbound               | SelectShellBackwardWord | Adjust the current selection to include the previous word using ShellBackwardWord     |
| Unbound               | SelectShellForwardWord  | Adjust the current selection to include the next word using ShellForwardWord          |
| Unbound               | SelectShellNextWord     | Adjust the current selection to include the next word using ShellNextWord             |
| Unbound               | SetMark                 | Mark the location of the cursor                                                       |

## Search functions

| Key      | Function                      | Description                                                                                |
| :------- | :---------------------------- | :----------------------------------------------------------------------------------------- |
| F3       | CharacterSearch               | Read a character and move the cursor to the next occurrence of that character              |
| Shift+F3 | CharacterSearchBackward       | Read a character and move the cursor to the previous occurrence of that character          |
| Unbound  | RepeatLastCharSearch          | Repeat the last recorded character search.                                                 |
| Unbound  | RepeatLastCharSearchBackwards | Repeat the last recorded character search in the opposite direction.                       |
| Unbound  | RepeatSearch                  | Repeat the last search.                                                                    |
| Unbound  | RepeatSearchBackward          | Repeat the last search, but in the opposite direction.                                     |
| Unbound  | SearchChar                    | Move to the next occurrence of the specified character.                                    |
| Unbound  | SearchCharBackward            | Move to the previous occurrence of the specified character.                                |
| Unbound  | SearchCharBackwardWithBackoff | Move to the previous occurrence of the specified character and then forward one character. |
| Unbound  | SearchCharWithBackoff         | Move to he next occurrence of the specified character and then back one character.         |
| Unbound  | SearchForward                 | Prompts for a search string and initiates a search upon AcceptLine.                        |

## User defined functions

| Key           | Function                   | Description                                         |
| :------------ | :------------------------- | :-------------------------------------------------- |
| Tab           | CustomAction               | User defined action                                 |
| Ctrl+q        | CustomAction               | User defined action                                 |
| Ctrl+f        | CustomAction               | User defined action                                 |
| Ctrl+t        | Fzf Provider Select        | Run fzf for current provider based on current token |
| Ctrl+r        | Fzf Reverse History Select | Run fzf to search through PSReadline history        |
| Alt+c         | Fzf Set Location           | Run fzf to select directory to set current location |
| Ctrl+b        | IntelliShell Bookmark      | Bookmarks current command                           |
| Ctrl+Spacebar | IntelliShell Search        | Searches for a bookmarked command                   |

## To learn

| Key           | Function                | Description                                                                                                                        |
| :------------ | :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| Alt+a         | SelectCommandArgument   | Make visual selection of the command arguments.                                                                                    |
| Alt+?         | WhatIsKey               | Show the key binding for the next chord entered                                                                                    |
| Ctrl+Alt+?    | ShowKeyBindings         | Show all key bindings                                                                                                              |
| Alt+h         | ShowParameterHelp       | Shows help for the parameter at the cursor.                                                                                        |
| Alt+0..9     | DigitArgument           | Start or accumulate a numeric argument to other functions                                                                          |
| Alt+-         | DigitArgument           | Start or accumulate a numeric argument to other functions                                                                          |
| Ctrl+l        | ClearScreen             | Clear the screen and redraw the current line at the top of the screen                                                              |
| Shift+Tab     | TabCompletePrevious     | Complete the input using the previous completion                                                                                   |
| Alt+F7        | ClearHistory            | Remove all items from the command line history (not PowerShell history)                                                            |
| Ctrl+]        | GotoBrace               | Go to matching brace                                                                                                               |
| Ctrl+w        | BackwardKillWord        | Move the text from the start of the current or previous word to the cursor to the kill ring                                        |
