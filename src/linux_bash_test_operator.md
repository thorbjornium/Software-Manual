# Bash

<!-- toc -->

## 🛠 Bash Test Operator Cheat Sheet

### 🔤 String Comparisons

| Operator | Meaning | Example |
|----------|---------|---------|
| `=` / `==` | Strings are equal | `[[ $a == "yes" ]]` |
| `!=` | Strings are NOT equal | `[[ $a != "no" ]]` |
| `=~` | Left side matches regex on right | `[[ $name =~ ^[A-Z] ]]` |
| `<` / `>` | Lexicographic (alphabetical) compare | `[[ $a < $b ]]` |

---

### 🔢 Integer Comparisons

(*Use `-` style operators for numbers*)

| Operator | Meaning | Example |
|----------|---------|---------|
| `-eq` | equal to | `[ $x -eq 5 ]` |
| `-ne` | not equal to | `[ $x -ne 10 ]` |
| `-lt` | less than | `[ $x -lt 3 ]` |
| `-le` | less than or equal to | `[ $x -le 7 ]` |
| `-gt` | greater than | `[ $x -gt 100 ]` |
| `-ge` | greater than or equal to | `[ $x -ge 18 ]` |

---

### 📂 File Tests

| Operator | Meaning | Example |
|----------|---------|---------|
| `-e` | file exists | `[ -e /path/file ]` |
| `-f` | regular file | `[ -f myfile.txt ]` |
| `-d` | directory exists | `[ -d /etc ]` |
| `-r` | file is readable | `[ -r secret.txt ]` |
| `-w` | file is writable | `[ -w /tmp/file ]` |
| `-x` | file is executable | `[ -x script.sh ]` |

---

### 🌀 Regex Examples with `=~`

| Pattern | Matches | Doesn’t Match |
|---------|---------|---------------|
| `^[A-Z]` | "Hello", "Zebra" | "apple", "1start" |
| `^[0-9]+$` | "123", "007" | "12a", "abc" |
| `foo\|bar` | "foo", "barbecue" | "baz" |
| `^[a-z]{3}$` | "cat", "dog" | "cats", "do" |
| `\.(jpg\|png)$` | "file.jpg", "img.png" | "pic.jpeg", "file.JPG" |

---

### 💡 Pro Tips

- Prefer `[[ ... ]]` over `[ ... ]` — it’s safer and supports `=~` regex matching.
- Don’t quote the regex pattern on the right side of `=~` — quoting forces a literal match.
- Prefix a test with `!` to invert it (logical NOT).
- Combine tests with `&&` (AND) and `||` (OR) for complex conditions.
