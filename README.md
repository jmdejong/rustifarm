# Rustifarm
This is the rust version of the asciifarm server.

This server is the replacement for the [python asciifarm server](https://github.com/jmdejong/asciifarm).

To join the game you need the [asciifarm client](https://github.com/jmdejong/asciifarm-client).

## About Asciifarm

![asciifarm screenshot](img/Screenshot_2020-04-12_11-31-20.png)

Asciifarm is a multiplayer RPG/farming game that is played in the terminal.

The intended use is to play this servers with a shared login (through ssh) but it can be layed in other contexts too.

Players can fight enemies and plant crops to gather resources.


## Installation/Running

Installation has been tested and confirmed to work on Linux and OpenBSD.
If anyone is able to run this on other operating systems, please tell me about the results.

### linux

Linux users can grab the binary from the releases page.

Apart from the binary, the `content/` directory in this repository is needed too.
This content directory should either be in the current working directory, or its location should be given with the `--content-dir` command line argument or the `ASCIIFARM_CONTENT_DIR` environment variable.

The released binaries may not always be up to date, so the make sure you have the latest version it can be better to follow the instructions for others instead.

### others

Install Rust and Cargo: https://www.rust-lang.org/tools/install

Run the command `cargo run` to compile and run asciifarm with all the default options.

Run the command `cargo build --release` to compile asciifarm and to create a binary which can then be run without cargo (the binary still needs the `content/` directory.
The binary will appear in the `target/release` directory, with the name `asciifarm`.


## Command line arguments

To see all command line arguments, pass the argument `--help`:

	$ ./asciifarm --help
	Rustifarm 0.2.0
	Asciifarm server in Rust

	USAGE:
		asciifarm [OPTIONS] --admins <admins>

	FLAGS:
		-h, --help       Prints help information
		-V, --version    Prints version information

	OPTIONS:
		-a, --address <address>...         A server type and address. Allowed server types: 'inet', 'unix', 'abstract'.
										Example: "inet:127.0.0.1:1234" or "abstract:rustifarm"
			--admins <admins>              The name(s) of the server admin(s) [env: USER=troido]
		-c, --content-dir <content-dir>    The directory in which the content specifying the world is (maps/encyclopaedia)
										[env: ASCIIFARM_CONTENT_DIR=]
		-s, --save-dir <save-dir>          The directory in which the savegames are [env: ASCIIFARM_SAVE_DIR=]
		-u, --user-dir <user-dir>          The directory in which the user sign-in data lives [env: ASCIIFARM_USER_DIR=]

