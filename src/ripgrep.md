# ripgrep  

ripgrep is a line-oriented search tool that recursively searches the current directory for a regex pattern.

Here are some examples of ripgrep usage:

`rg '// TODO'`

Searches for the string // TODO in the current directory.

`rg -i -w "error|warning"`

Searches for lines containing either error or warning ignoring case, and requiring it to be a whole word in the current directory.

`rg "^TODO"`

Searches for lines that start with `TODO` in the current directory.

`rg "\.jpg$"`

Searches for files ending with `.jpg` in the current directory.

`rg "\d+"`

Searches for lines containing numbers in the current directory.

`rg "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"`

Searches for lines containing email addresses in the current directory.

`rg "test.*"`

Searches for lines containing the string "test." followed by any characters in the current directory.

`rg "\b\w{5}\b"`

Searches for lines containing exactly 5 characters in the current directory.

`rg "[^a]+"`

Searches for lines containing characters other than `a` in the current directory.

`rg "\d+[a-zA-Z]"`

Searches for lines containing a number followed by a letter in the current directory.

`rg --type js --type py "function"`

Searches for lines containing the string "function" in files with the extension `.js` or `.py` in the current directory.

## Common options for ripgrep

- `-i --ignore-case`: When searching for a pattern, ignore case differences. That is `rg -i fast` matches `fast`, `fASt`, `FAST`, etc.  

- `-S --smart-case`: This is similar to `--ignore-case`, but disables itself if the pattern contains any uppercase letters. Usually this flag is put into alias or a config file.  

- `-F --fixed-strings`: Disable regular expression matching and treat the pattern as a literal string.  

- `-w --word-regexp`: Require that all matches of the pattern be surrounded by word boundaries. That is, given `pattern`, the `--word-regexp` flag will cause ripgrep to behave as if `pattern` were actually `\b(?:pattern)\b`.  

- `-c --count`: Report a count of total matched lines.  

- `-l --files-with-matches`: Only print the names of files that contain matches.  

- `--files-without-match`: Only print the names of files that do not contain matches.  

- `--files`: Print the files that ripgrep *would* search, but don't actually search them.  

- `-a --text`: Search binary files as if they were plain text.  

- `-U --multiline`: Permit matches to span multiple lines.  

- `-z --search-zip`: Search compressed files (gzip, bzip2, lzma, xz, lz4, brotli, zstd). This is disabled by default.  

- `-C --context`: Show the lines surrounding a match.  

- `--sort path`: Force ripgrep to sort its output by file name. (This disables parallelism, so it might be slower.)  

- `-L --follow`: Follow symbolic links while recursively searching.  

- `-M --max-columns`: Limit the length of lines printed by ripgrep.  

- `--debug`: Shows ripgrep's debug output. This is useful for understanding why a particular file might be ignored from search, or what kinds of configuration ripgrep is loading from the environment.  

More details can be found <a href="https://github.com/BurntSushi/ripgrep" target="blank">here</a>  
