# fd

<!-- toc -->

Install

{{#tabs global="install" }}
{{#tab name="Winget" }}

```Powershell
winget install sharkdp.fd
```  

{{#endtab }}
{{#tab name="Scoop" }}

```Powershell
Scoop fd
```  

{{#endtab }}

{{#endtabs }}

## Usage

How to use

First, to get an overview of all available command line options, you can either run fd -h for a concise help message or fd --help for a more detailed version.
Simple search

fd is designed to find entries in your filesystem. The most basic search you can perform is to run fd with a single argument: the search pattern. For example, assume that you want to find an old script of yours (the name included netflix):

`fd netfl`  
Software/python/imdb-ratings/netflix-details.py

If called with just a single argument like this, fd searches the current directory recursively for any entries that contain the pattern netfl.
Regular expression search

The search pattern is treated as a regular expression. Here, we search for entries that start with x and end with rc:

`cd \etc`  
`fd '^x.*rc$'`  
X11/xinit/xinitrc
X11/xinit/xserverrc

The regular expression syntax used by fd is documented here.
Specifying the root directory

If we want to search a specific directory, it can be given as a second argument to fd:

`fd passwd \etc`  
etc\default\passwd
etc\pam.d\passwd
etc\passwd

List all files, recursively

fd can be called with no arguments. This is very useful to get a quick overview of all entries in the current directory, recursively (similar to ls -R):

`cd fd\tests`  
`fd`  
testenv
testenv/mod.rs
tests.rs

If you want to use this functionality to list all files in a given directory, you have to use a catch-all pattern such as . or ^:

`cd fd\tests`  
`fd .`  
testenv
testenv/mod.rs
tests.rs

Searching for a particular file extension

Often, we are interested in all files of a particular type. This can be done with the -e (or --extension) option. Here, we search for all Markdown files in the fd repository:

`cd fd`  
`fd -e md`  
CONTRIBUTING.md
README.md

The -e option can be used in combination with a search pattern:

`fd -e rs mod`  
src/fshelper/mod.rs
src/lscolors/mod.rs
tests/testenv/mod.rs

Searching for a particular file name

To find files with exactly the provided search pattern, use the -g (or --glob) option:

`fd -g libc.so /usr`  
/usr/lib32/libc.so
/usr/lib/libc.so

Hidden and ignored files

By default, fd does not search hidden directories and does not show hidden files in the search results. To disable this behavior, we can use the -H (or --hidden) option:

`fd pre-commit`  
`fd -H pre-commit`  
.git/hooks/pre-commit.sample

If we work in a directory that is a Git repository (or includes Git repositories), fd does not search folders (and does not show files) that match one of the .gitignore patterns. To disable this behavior, we can use the -I (or --no-ignore) option:

`fd num_cpu`  
`fd -I num_cpu`  
target/debug/deps/libnum_cpus-f5ce7ef99006aa05.rlib

To really search all files and directories, simply combine the hidden and ignore features to show everything (-HI) or use -u/--unrestricted.
Matching the full path

By default, fd only matches the filename of each file. However, using the --full-path or -p option, you can match against the full path.

`fd -p -g '**/.git/config'`  
`fd -p '.*/lesson-\d+/[a-z]+.(jpg|png)'`  

Command execution

Instead of just showing the search results, you often want to do something with them. fd provides two ways to execute external commands for each of your search results:

    The -x/--exec option runs an external command for each of the search results (in parallel).
    The -X/--exec-batch option launches the external command once, with all search results as arguments.

Examples

Recursively find all zip archives and unpack them:

`fd -e zip -x unzip`  

If there are two such files, file1.zip and backup/file2.zip, this would execute unzip file1.zip and unzip backup/file2.zip. The two unzip processes run in parallel (if the files are found fast enough).

Find all \*.h and \*.cpp files and auto-format them inplace with clang-format -i:

`fd -e h -e cpp -x clang-format -i`  

Note how the -i option to clang-format can be passed as a separate argument. This is why we put the -x option last.

Find all test_*.py files and open them in your favorite editor:

`fd -g 'test_*.py' -X vim`  

Note that we use capital -X here to open a single vim instance. If there are two such files, test_basic.py and lib/test_advanced.py, this will run vim test_basic.py lib/test_advanced.py.

To see details like file permissions, owners, file sizes etc., you can tell fd to show them by running ls for each result:

`fd … -X ls -lhd --color=always`  

This pattern is so useful that fd provides a shortcut. You can use the -l/--list-details option to execute ls in this way: `fd … -l`  

The -X option is also useful when combining fd with ripgrep (rg) in order to search within a certain class of files, like all C++ source files:

`fd -e cpp -e cxx -e h -e hpp -X rg 'std::cout <<'`  

Convert all \*.jpg files to \*.png files:

`fd -e jpg -x convert {} {.}.png`  

Here, {} is a placeholder for the search result. {.} is the same, without the file extension. See below for more details on the placeholder syntax.

The terminal output of commands run from parallel threads using -x will not be interlaced or garbled, so fd -x can be used to rudimentarily parallelize a task run over many files. An example of this is calculating the checksum of each individual file within a directory.

`fd -tf -x md5sum > file_checksums.txt`  

Placeholder syntax

The -x and -X options take a command template as a series of arguments (instead of a single string). If you want to add additional options to fd after the command template, you can terminate it with a \;.

The syntax for generating commands is similar to that of GNU Parallel:

    {}: A placeholder token that will be replaced with the path of the search result (documents/images/party.jpg).
    {.}: Like {}, but without the file extension (documents/images/party).
    {/}: A placeholder that will be replaced by the basename of the search result (party.jpg).
    {//}: The parent of the discovered path (documents/images).
    {/.}: The basename, with the extension removed (party).

If you do not include a placeholder, fd automatically adds a {} at the end.
Parallel vs. serial execution

For -x/--exec, you can control the number of parallel jobs by using the -j/--threads option. Use --threads=1 for serial execution.
Excluding specific files or directories

Sometimes we want to ignore search results from a specific subdirectory. For example, we might want to search all hidden files and directories (-H) but exclude all matches from .git directories. We can use the -E (or --exclude) option for this. It takes an arbitrary glob pattern as an argument:

`fd -H -E .git …`  

We can also use this to skip mounted directories:

`fd -E /mnt/external-drive …`  

.. or to skip certain file types:

`fd -E '*.bak' …`  

To make exclude-patterns like these permanent, you can create a .fdignore file. They work like .gitignore files, but are specific to fd. For example:

`cat ~/.fdignore`  
/mnt/external-drive
*.bak

Note

fd also supports .ignore files that are used by other programs such as rg or ag.

If you want fd to ignore these patterns globally, you can put them in fd's global ignore file. This is usually located in ~/.config/fd/ignore in macOS or Linux, and %APPDATA%\fd\ignore in Windows.

You may wish to include .git/ in your fd/ignore file so that .git directories, and their contents are not included in output if you use the --hidden option.
Deleting files

You can use fd to remove all files and directories that are matched by your search pattern. If you only want to remove files, you can use the --exec-batch/-X option to call rm. For example, to recursively remove all .DS_Store files, run:

`fd -H '^\.DS_Store$' -tf -X rm`

If you are unsure, always call fd without -X rm first. Alternatively, use rms "interactive" option:

`fd -H '^\.DS_Store$' -tf -X rm -i`

If you also want to remove a certain class of directories, you can use the same technique. You will have to use rms --recursive/-r flag to remove directories.

Note

There are scenarios where using fd … -X rm -r can cause race conditions: if you have a path like …/foo/bar/foo/… and want to remove all directories named foo, you can end up in a situation where the outer foo directory is removed first, leading to (harmless) "'foo/bar/foo': No such file or directory" errors in the rm call.
Command-line options

This is the output of fd -h. To see the full set of command-line options, use fd --help which also includes a much more detailed help text.

Usage: fd [OPTIONS] [pattern] [path]...

Arguments:
  [pattern]  the search pattern (a regular expression, unless '--glob' is used; optional)
  [path]...  the root directories for the filesystem search (optional)

| Options                      | Description                                                                                                                                 |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| -H, --hidden                 | Search hidden files and directories                                                                                                         |
| -I, --no-ignore              | Do not respect .(git\|fd) ignore files                                                                                                      |
| -s, --case-sensitive         | Case-sensitive search (default: smart case)                                                                                                 |
| -i, --ignore-case            | Case-insensitive search (default: smart case)                                                                                               |
| -g, --glob                   | Glob-based search (default: regular expression)                                                                                             |
| -a, --absolute-path          | Show absolute instead of relative paths                                                                                                     |
| -l, --list-details           | Use a long listing format with file metadata                                                                                                |
| -L, --follow                 | Follow symbolic links                                                                                                                       |
| -p, --full-path              | Search full abs. path (default: filename only)                                                                                              |
| -d, --max-depth \<depth>     | Set maximum search depth (default: none)                                                                                                    |
| -E, --exclude \<pattern>      | Exclude entries that match the given glob pattern                                                                                           |
| -t, --type \<filetype>        | Filter by type: file (f), directory (d/dir), symlink (l),executable (x), empty (e), socket (s), pipe (p), char-device (c), block-device (b) |
| -e, --extension \<ext>        | Filter by file extension                                                                                                                    |
|                              |                                                                                                                                             |
| -S, --size \<size>            | Limit results based on the size of files                                                                                                    |
| --changed-within <date\|dur> | Filter by file modification time (newer than)                                                                                               |
| --changed-before <date\|dur> | Filter by file modification time (older than)                                                                                               |
|                              |                                                                                                                                             |
| -o, --owner <user:group>     | Filter by owning user and/or group                                                                                                          |
| --format \<fmt>               | Print results according to template                                                                                                         |
| -x, --exec \<cmd>...          | Execute a command for each search result                                                                                                    |
| -X, --exec-batch \<cmd>...    | Execute a command with all search results at once                                                                                           |
| -c, --color \<when>           | When to use colors [default: auto] [possible values: auto, always, never]                                                                   |
| --hyperlink[=\<when>]         | Add hyperlinks to output paths [default: never] [possible values: auto, always, never]                                                      |
| -h, --help                   | Print help (see more with '--help')                                                                                                         |
| -V, --version                | Print version                                                                                                                               |

## Full output from --help

Usage: fd.exe [OPTIONS] [pattern] [path]...

Arguments:
  [pattern]
          the search pattern which is either a regular expression (default) or a glob pattern (if
          --glob is used). If no pattern has been specified, every entry is considered a match. If
          your pattern starts with a dash (-), make sure to pass '--' first, or it will be
          considered as a flag (fd -- '-foo').

  [path]...
          The directory where the filesystem search is rooted (optional). If omitted, search the
          current working directory.

Options:
  -H, --hidden
          Include hidden directories and files in the search results (default: hidden files and
          directories are skipped). Files and directories are considered to be hidden if their
          name starts with a `.` sign (dot). Any files or directories that are ignored due to the
          rules described by --no-ignore are still ignored unless otherwise specified. The flag
          can be overridden with --no-hidden.

  -I, --no-ignore
          Show search results from files and directories that would otherwise be ignored by
          '.gitignore', '.ignore', '.fdignore', or the global ignore file, The flag can be
          overridden with --ignore.

      --no-ignore-vcs
          Show search results from files and directories that would otherwise be ignored by
          '.gitignore' files. The flag can be overridden with --ignore-vcs.

      --no-require-git
          Do not require a git repository to respect gitignores. By default, fd will only respect
          global gitignore rules, .gitignore rules, and local exclude rules if fd detects that you
          are searching inside a git repository. This flag allows you to relax this restriction
          such that fd will respect all git related ignore rules regardless of whether you're
          searching in a git repository or not.
          
          This flag can be disabled with --require-git.

      --no-ignore-parent
          Show search results from files and directories that would otherwise be ignored by
          '.gitignore', '.ignore', or '.fdignore' files in parent directories.

  -u, --unrestricted...
          Perform an unrestricted search, including ignored and hidden files. This is an alias for
          '--no-ignore --hidden'.

  -s, --case-sensitive
          Perform a case-sensitive search. By default, fd uses case-insensitive searches, unless
          the pattern contains an uppercase character (smart case).

  -i, --ignore-case
          Perform a case-insensitive search. By default, fd uses case-insensitive searches, unless
          the pattern contains an uppercase character (smart case).

  -g, --glob
          Perform a glob-based search instead of a regular expression search.

      --regex
          Perform a regular-expression based search (default). This can be used to override
          --glob.

  -F, --fixed-strings
          Treat the pattern as a literal string instead of a regular expression. Note that this
          also performs substring comparison. If you want to match on an exact filename, consider
          using '--glob'.

      --and <pattern>
          Add additional required search patterns, all of which must be matched. Multiple
          additional patterns can be specified. The patterns are regular expressions, unless
          '--glob' or '--fixed-strings' is used.

  -a, --absolute-path
          Shows the full path starting from the root as opposed to relative paths. The flag can be
          overridden with --relative-path.

  -l, --list-details
          Use a detailed listing format like 'ls -l'. This is basically an alias for '--exec-batch
          ls -l' with some additional 'ls' options. This can be used to see more metadata, to show
          symlink targets and to achieve a deterministic sort order.

  -L, --follow
          By default, fd does not descend into symlinked directories. Using this flag, symbolic
          links are also traversed. Flag can be overridden with --no-follow.

  -p, --full-path
          By default, the search pattern is only matched against the filename (or directory name).
          Using this flag, the pattern is matched against the full (absolute) path. Example:
            fd --glob -p '**/.git/config'

  -0, --print0
          Separate search results by the null character (instead of newlines). Useful for piping
          results to 'xargs'.

  -d, --max-depth <depth>
          Limit the directory traversal to a given depth. By default, there is no limit on the
          search depth.

      --min-depth <depth>
          Only show search results starting at the given depth. See also: '--max-depth' and
          '--exact-depth'

      --exact-depth <depth>
          Only show search results at the exact given depth. This is an alias for '--min-depth
          <depth> --max-depth <depth>'.

  -E, --exclude <pattern>
          Exclude files/directories that match the given glob pattern. This overrides any other
          ignore logic. Multiple exclude patterns can be specified.

          Examples: 
            --exclude '*.pyc' 
            --exclude node_modules

      --prune
          Do not traverse into directories that match the search criteria. If you want to exclude
          specific directories, use the '--exclude=ΓÇª' option.

  -t, --type <filetype>
          Filter the search by type:
            'f' or 'file':         regular files
            'd' or 'dir' or 'directory':    directories
            'l' or 'symlink':      symbolic links
            's' or 'socket':       socket
            'p' or 'pipe':         named pipe (FIFO)
            'b' or 'block-device': block device
            'c' or 'char-device':  character device

            'x' or 'executable':   executables 
            'e' or 'empty':        empty files or directories
          
          This option can be specified more than once to include multiple file types. Searching
          for '--type file --type symlink' will show both regular files as well as symlinks. Note
          that the 'executable' and 'empty' filters work differently: '--type executable' implies
          '--type file' by default. And '--type empty' searches for empty files and directories,
          unless either '--type file' or '--type directory' is specified in addition.
          
          Examples: 
            - Only search for files: 
                fd --type file ΓÇª 
                fd -tf ΓÇª 
            - Find both files and symlinks 
                fd --type file --type symlink ΓÇª 
                fd -tf -tl ΓÇª 
            - Find executable files: 
                fd --type executable 
                fd -tx 
            - Find empty files: 
                fd --type empty --type file 
                fd -te -tf 
            - Find empty directories: 
                fd --type empty --type directory 
                fd -te -td

  -e, --extension <ext>
          (Additionally) filter search results by their file extension. Multiple allowable file
          extensions can be specified.

          If you want to search for files without extension, you can use the regex '^[^.]+$' as a
          normal search pattern.

  -S, --size <size>
          Limit results based on the size of files using the format <+-><NUM><UNIT>.
             '+': file size must be greater than or equal to this
             '-': file size must be less than or equal to this

          If neither '+' nor '-' is specified, file size must be exactly equal to this.
             'NUM':  The numeric size (e.g. 500)
             'UNIT': The units for NUM. They are not case-sensitive.
          Allowed unit values:
              'b':  bytes
              'k':  kilobytes (base ten, 10^3 = 1000 bytes)
              'm':  megabytes
              'g':  gigabytes
              't':  terabytes
              'ki': kibibytes (base two, 2^10 = 1024 bytes)
              'mi': mebibytes
              'gi': gibibytes
              'ti': tebibytes

      --changed-within <date|dur>
          Filter results based on the file modification time. Files with modification times
          greater than the argument are returned. The argument can be provided as a specific point
          in time (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min). If the
          time is not specified, it defaults to 00:00:00. '--change-newer-than', '--newer', or
          '--changed-after' can be used as aliases.
          
          Examples: 
              --changed-within 2weeks 
              --change-newer-than '2018-10-27 10:00:00' 
              --newer 2018-10-27 
              --changed-after 1day

      --changed-before <date|dur>
          Filter results based on the file modification time. Files with modification times less
          than the argument are returned. The argument can be provided as a specific point in time
          (YYYY-MM-DD HH:MM:SS or @timestamp) or as a duration (10h, 1d, 35min).
          '--change-older-than' or '--older' can be used as aliases.
          
          Examples: 
              --changed-before '2018-10-27 10:00:00' 
              --change-older-than 2weeks 
              --older 2018-10-27

      --format <fmt>
          Print results according to template

  -x, --exec <cmd>...
          Execute a command for each search result in parallel (use --threads=1 for sequential
          command execution). There is no guarantee of the order commands are executed in, and the
          order should not be depended upon. All positional arguments following --exec are
          considered to be arguments to the command - not to fd. It is therefore recommended to
          place the '-x'/'--exec' option last.
          The following placeholders are substituted before the command is executed:
            '{}':   path (of the current search result)
            '{/}':  basename
            '{//}': parent directory
            '{.}':  path without file extension
            '{/.}': basename without file extension
            '{{':   literal '{' (for escaping)
            '}}':   literal '}' (for escaping)

          If no placeholder is present, an implicit "{}" at the end is assumed.
          
          Examples:
          
            - find all *.zip files and unzip them:
          
                fd -e zip -x unzip
          
            - find *.h and *.cpp files and run "clang-format -i .." for each of them:
          
                fd -e h -e cpp -x clang-format -i
          
            - Convert all *.jpg files to *.png files:
          
                fd -e jpg -x convert {} {.}.png

  -X, --exec-batch <cmd>...
          Execute the given command once, with all search results as arguments.
          The order of the arguments is non-deterministic, and should not be relied upon.
          One of the following placeholders is substituted before the command is executed:
            '{}':   path (of all search results)
            '{/}':  basename
            '{//}': parent directory
            '{.}':  path without file extension
            '{/.}': basename without file extension
            '{{':   literal '{' (for escaping)
            '}}':   literal '}' (for escaping)

          If no placeholder is present, an implicit "{}" at the end is assumed.
          
          Examples:
          
            - Find all test_*.py files and open them in your favorite editor:
          
                fd -g 'test_*.py' -X vim
          
            - Find all *.rs files and count the lines with "wc -l ...":
          
                fd -e rs -X wc -l

      --batch-size <size>
          Maximum number of arguments to pass to the command given with -X. If the number of
          results is greater than the given size, the command given with -X is run again with
          remaining arguments. A batch size of zero means there is no limit (default), but note
          that batching might still happen due to OS restrictions on the maximum length of command
          lines.
          
          [default: 0]

      --ignore-file <path>
          Add a custom ignore-file in '.gitignore' format. These files have a low precedence.

  -c, --color <when>
          Declare when to use color for the pattern match output

          [default: auto]

          Possible values:
          - auto:   show colors if the output goes to an interactive console (default)
          - always: always use colorized output
          - never:  do not use colorized output

      --hyperlink[=<when>]
          Add a terminal hyperlink to a file:// url for each path in the output.
          
          Auto mode  is used if no argument is given to this option.
          
          This doesn't do anything for --exec and --exec-batch.
          
          [default: never]

          Possible values:
          - auto:   Use hyperlinks only if color is enabled
          - always: Always use hyperlinks when printing file paths
          - never:  Never use hyperlinks

  -j, --threads <num>
          Set number of threads to use for searching & executing (default: number of available CPU
          cores)

      --max-results <count>
          Limit the number of search results to 'count' and quit immediately.

  -1
          Limit the search to a single result and quit immediately. This is an alias for
          '--max-results=1'.

  -q, --quiet
          When the flag is present, the program does not print anything and will return with an
          exit code of 0 if there is at least one match. Otherwise, the exit code will be 1.
          '--has-results' can be used as an alias.

      --show-errors
          Enable the display of filesystem errors for situations such as insufficient permissions
          or dead symlinks.

      --base-directory <path>
          Change the current working directory of fd to the provided path. This means that search
          results will be shown with respect to the given base path. Note that relative paths
          which are passed to fd via the positional <path> argument or the '--search-path' option
          will also be resolved relative to this directory.

      --path-separator <separator>
          Set the path separator to use when printing file paths. The default is the OS-specific
          separator ('/' on Unix, '\' on Windows).

      --search-path <search-path>
          Provide paths to search as an alternative to the positional <path> argument. Changes the
          usage to `fd [OPTIONS] --search-path <path> --search-path <path2> [<pattern>]`

      --strip-cwd-prefix[=<when>]
          By default, relative paths are prefixed with './' when -x/--exec, -X/--exec-batch, or
          -0/--print0 are given, to reduce the risk of a path starting with '-' being treated as a
          command line option. Use this flag to change this behavior. If this flag is used without
          a value, it is equivalent to passing "always".

          Possible values:
          - auto:   Use the default behavior
          - always: Always strip the ./ at the beginning of paths
          - never:  Never strip the ./

      --one-file-system
          By default, fd will traverse the file system tree as far as other options dictate. With
          this flag, fd ensures that it does not descend into a different file system than the one
          it started in. Comparable to the -mount or -xdev filters of find(1).

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Bugs can be reported on GitHub: <https://github.com/sharkdp/fd/issues>
