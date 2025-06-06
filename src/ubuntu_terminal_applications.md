# Terminal Applications

<!-- toc -->

## Quick Install

```bash

sudo apt install bat zoxide fd-find ripgrep

```

### bat

Important: If you install bat this way, please note that the executable may be installed as batcat instead of bat (due to a name clash with another package). You can set up a bat -> batcat symlink or alias to prevent any issues that may come up because of this and to be consistent with other distributions:

```bash

mkdir -p ~/.local/bin
ln -s /usr/bin/batcat ~/.local/bin/bat

```

More info at [https://github.com/sharkdp/bat](https://github.com/sharkdp/bat)

### fzf

**To get the most up-to-date version on Ubuntu Server 24.04:**

```bash

git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
~/.fzf/install

```

Add the following line to your shell configuration file.

bash:

```bash

eval "$(fzf --bash)"

```

More info at [https://github.com/junegunn/fzf](https://github.com/junegunn/fzf)

### zoxide

Set up the repo:

```bash

curl -fsSL https://apt.cli.rs/pubkey.asc | sudo tee -a /usr/share/keyrings/rust-tools.asc
curl -fsSL https://apt.cli.rs/rust-tools.list | sudo tee /etc/apt/sources.list.d/rust-tools.list
sudo apt update

```

You might need to run `sudo apt install zoxide` again.

bash:

Add this to the **end** of your config file (usually ~/.bashrc):

```bash

eval "$(zoxide init bash)"

```

More info at [https://github.com/ajeetdsouza/zoxide](https://github.com./ajeetdsouza/zoxide)

### intelli-shell

Download repo:

```bash

curl -sSf https://raw.githubusercontent.com/lasantosr/intelli-shell/main/install.sh | bash

```

Enable hotkeys:

```bash

mkdir -p ~/.local/share/intelli-shell/bin
curl -sSf https://raw.githubusercontent.com/lasantosr/intelli-shell/main/intelli-shell.sh > ~/.local/share/intelli-shell/bin/intelli-shell.sh

```

More info at [https://github.com/lasantosr/intelli-shell](https://github.com/lasantosr/intelli-shell)

### fd

Note that the binary is called fdfind as the binary name fd is already used by another package. It is recommended that after installation, you add a link to fd by executing command..

```bash

ln -s $(which fdfind) ~/.local/bin/fd

```

.. in order to use fd in the same way as in this documentation. Make sure that $HOME/.local/bin is in your $PATH.

You can use *fd* to generate input for the command-line fuzzy finder fzf:

```bash

export FZF_DEFAULT_COMMAND='fd --type file' export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"

```

Then, you can type `vim <Ctrl-T>` on your terminal to open fzf and search through the fd-results.

Alternatively, you might like to follow symbolic links and include hidden files (but exclude `.git` folders):

```bash

export FZF_DEFAULT_COMMAND='fd --type file --follow --hidden --exclude .git'

```

You can even use fd's colored output inside fzf by setting:

```bash

export FZF_DEFAULT_COMMAND="fd --type file --color=always" export FZF_DEFAULT_OPTS="--ansi"

```

More info at [https://github.com/sharkdp/fd](https://github.com/sharkdp/fd) and see the [Tips section](https://github.com/junegunn/fzf#tips) of the fzf README.

### ripgrep

Info at [https://github.com/BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep) and user guide here: [https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md)

### eza

Eza is available from [deb.gierens.de](http://deb.gierens.de). The GPG public key is in this repo under [deb.asc](https://github.com/eza-community/eza/blob/main/deb.asc).

First make sure you have the `gpg` command, and otherwise install it via:

```bash

sudo apt update sudo apt install -y gpg

```

Then install eza via:

```bash

sudo mkdir -p /etc/apt/keyrings wget -qO- https://raw.githubusercontent.com/eza-community/eza/main/deb.asc | sudo gpg --dearmor -o /etc/apt/keyrings/gierens.gpg echo "deb [signed-by=/etc/apt/keyrings/gierens.gpg] http://deb.gierens.de stable main" | sudo tee /etc/apt/sources.list.d/gierens.list sudo chmod 644 /etc/apt/keyrings/gierens.gpg /etc/apt/sources.list.d/gierens.list sudo apt update sudo apt install -y eza

```

*Note*: In strict apt environments, you may need to add the target: `echo "deb [arch=amd64 signed-by=...`

More info at [https://github.com/eza-community/eza](https://github.com/eza-community/eza)
