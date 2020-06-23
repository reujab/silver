# silver
A cross-shell customizable powerline-like prompt heavily inspired by [Agnoster](https://github.com/agnoster/agnoster-zsh-theme). The faster rust port of [bronze](https://github.com/reujab/bronze).<br/>
![](./sleep.png)

## How does it work?
Unlike most shell prompts, silver is not written in shell script, but entirely in Rust.

When `silver init` is run, it outputs shell code that sets your prompt to run `silver prompt`, which outputs the actual prompt. The `silver prompt` command relies on environment variables for configuration.

## Getting started
Since silver is not written in shell script, it should theoretically be compatible with any shell, but the three supported shells are Bash, Zsh, and fish.

### Icons
To be able to use the custom icons (which are enabled by default), you must patch your font or install a pre-patched font from [Nerd Fonts](https://github.com/ryanoasis/nerd-fonts).

### Installation
#### From source
* install and setup [Rust](https://www.rust-lang.org/)
* run `cargo install silver`

#### From pre-compiled binary
* download a binary on the [releases page](https://github.com/reujab/silver/releases)
* add binary to `PATH` environment variable

#### macOS
On macOS, you will have to do a bit more:
* install [Homebrew](https://brew.sh/)
* run `brew install coreutils`
* run `brew install openssl`
* add `alias date="gdate"` to your shell rc

### Configuration
Now that you have silver installed, you need to configure it. To have your prompt look like the one in the screenshot above, add this to your `~/.bashrc`/`~/.zshrc`:
```sh
SILVER=(status:black:white dir:blue:black git:green:black cmdtime:magenta:black)
export SILVER_SHELL=zsh # or bash
```

Or add the following to your `~/.config/fish/config.fish`:
```fish
set SILVER status:black:white dir:blue:black git:green:black cmdtime:magenta:black
set -x SILVER_SHELL fish
```

Now that silver is configured, you need to evaluate its bootstrap code.

`~/.bashrc`/`~/.zshrc`:
```sh
eval "$(silver init)"
```

`~/.config/fish/config.fish`:
```fish
eval (silver init)
```

## Documentation
Documentation is available on [the wiki](https://github.com/reujab/silver/wiki).

## Project structure
* [`src/`](src)
	* [`modules/`](src/modules)
		* [`cmdtime.rs`](src/modules/cmdtime.rs)
			* source code for the [`cmdtime`](https://github.com/reujab/silver/wiki/Command-Time) module
		* [`dir.rs`](src/modules/dir.rs)
			* source code for the [`dir`](https://github.com/reujab/silver/wiki/Directory) module
		* [`env.rs`](src/modules/env.rs)
			* source code for the [`env`](https://github.com/reujab/silver/wiki/Environment-Variable) module
		* [`git.rs`](src/modules/git.rs)
			* source code for the [`git`](https://github.com/reujab/silver/wiki/Git) module
		* [`mod.rs`](src/modules/mod.rs)
			* handles modules
		* [`os.rs`](src/modules/os.rs)
			* source code for the [`os`](https://github.com/reujab/silver/wiki/OS) module
		* [`status.rs`](src/modules/status.rs)
			* source code for the [`status`](https://github.com/reujab/silver/wiki/Status) module
		* [`time.rs`](src/modules/time.rs)
			* source code for the [`time`](https://github.com/reujab/silver/wiki/Time) module
		* [`user.rs`](src/modules/user.rs)
			* source code for the [`user`](https://github.com/reujab/silver/wiki/User) module
		* [`virtualenv.rs`](src/modules/virtualenv.rs)
			* source code for the [`virtualenv`](https://github.com/reujab/silver/wiki/virtualenv) module
	* [`icons.rs`](src/icons.rs)
		* processes icons, separators, and Unicode
	* [`init.bash`](src/init.bash)
		* bootstrap code for Bash
	* [`init.fish`](src/init.fish)
		* bootstrap code for fish
	* [`init.zsh`](src/init.zsh)
		* bootstrap code for Zsh
	* [`main.rs`](src/main.rs)
		* parses command line arguments
	* [`print.rs`](src/print.rs)
		* prints prompt segments
	* [`sh.rs`](src/sh.rs)
		* shell-specific code
