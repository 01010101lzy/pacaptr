# pacaptr

`pacaptr` is a Rust port of [icy/pacapt], a wrapper for many package managers with pacman-style command syntax.

Run `pacaptr -Syu` on the distro of your choice!

## Contents

- [pacaptr](#pacaptr)
  - [Contents](#contents)
  - [Supported Package Managers](#supported-package-managers)
  - [Motivation & Current Status](#motivation--current-status)
  - [Running & Building](#running--building)
  - [General Tips](#general-tips)
  - [Platform-Specific Tips](#platform-specific-tips)

## Supported Package Managers

- `macOS/homebrew`
- `Windows/chocolatey`
- `Debian/dpkg`
- `Alpine/apk`

Notes:

- Support for more package managers will be added Soon™.
- Don't miss the [general](#general-tips) & [platform-specific](#platform-specific-tips) tips below!

## Motivation & Current Status

Coming from `Arch Linux` to `macOS`, I really like the idea of having an automated version of [Pacman Rosetta] for making common package managing tasks less of a travail thanks to the concise `pacman` syntax.

Initially, I found [icy/pacapt] which does just that, and I made this project to improve `pacapt`'s `homebrew` (especially `cask`) support. (See [pacapt/#117].)

After some discussions in [pacapt/#126], I decided to rewrite the project in Rust to improve readability, testing, etc.

Now the implementations of different package managers are all placed in `./src/packmanager` folder, with names like `homebrew.rs`.

## Running & Building

We currently provide `cargo install` only.
PPAs might be added when appropriate.

To play along at home:

```bash
# First you'll need to download the source:
git clone https://github.com/rami3l/pacaptr.git
cd pacaptr

# To run:
cargo run -- -S curl

# To install:
cargo install --path .

# To uninstall:
cargo uninstall pacaptr
```

## General Tips

- Additional flags support:
  - The flags after a `--` will be passed directly to the underlying package manager:

    ```bash
    pacaptr -h
    # USAGE:
    #     pacaptr [FLAGS] [KEYWORDS]... [-- <ADDITIONAL_FLAGS>...]

    pacaptr -S curl docker --dryrun -- --proxy=localhost:1234
    # Pending: foo install curl --proxy=localhost:1234
    # Pending: foo install docker --proxy=localhost:1234
    ```

    Here `foo` is the name of your package manager.
    (The actual output is platform-specific, which largely depends on if `foo` can actually read the flags given.)

- `--dryrun`, `--dry-run`: Use this flag to just print out the command to be executed
  (sometimes with a --dry-run flag to activate the package manager's dryrun option).

  - `Pending` means that the command execution is blocked (a dry run or prompted to continue),
  while `Running` means that it is running.

  - Some query commands might still be run, but anything "big" should have been stopped from running, eg. installation.
    For instance:

    ```bash
    # Nothing will be installed,
    # as `brew install curl` won't run:
    pacaptr -S curl --dryrun
    # Pending: brew install curl

    # Nothing will be deleted here,
    # but `brew cleanup --dry-run` is actually running:
    pacaptr -Sc --dryrun
    # Running: brew cleanup --dry-run
    # .. (showing the files to be removed)

    # To remove the forementioned files,
    # run the command above again without `--dryrun`:
    pacaptr -Sc
    # Running: brew cleanup
    # .. (cleaning up)
    ```

- `--yes`, `--noconfirm`, `--no-confirm`:
  Use this flag to trigger the corresponding flag of your package manager (if possible) in order to answer "yes" to every incoming question.
  - This option is useful when you don't want to be asked during installation, for example.
  - ... But it can be potentially dangerous if you don't know what you're doing!

## Platform-Specific Tips

- `Homebrew` support: Please note that this is for macOS only, `Linuxbrew` is currently not supported.

  - Automatic `brew cask` invocation: implemented for `-S`, `-R`, `-Su`, and more.

    ```bash
    pacaptr -S curl --dryrun
    # Pending: brew install curl

    pacaptr -S gimp --dryrun
    # Pending: brew cask install gimp
    ```

  - The use of `brew cask` commands can also be enforced by adding a `--cask` flag. Useful when a bottle and a cask share the same name, eg. `docker`.

  - To use `-Rs`, you need to install [rmtree] first:

    ```bash
    brew tap beeftornado/rmtree
    ```

- `Chocolatey` support: Don't forget to run in an elevated shell! You can do this easily with tools like [gsudo].

[Pacman Rosetta]: https://wiki.archlinux.org/index.php/Pacman/Rosetta
[icy/pacapt]: https://github.com/icy/pacapt
[pacapt/#117]: https://github.com/icy/pacapt/issues/117
[pacapt/#126]: https://github.com/icy/pacapt/issues/126
[rmtree]: https://github.com/beeftornado/homebrew-rmtree
[gsudo]: https://github.com/gerardog/gsudo
[rs-dev]: https://github.com/rami3l/pacaptr/tree/rs-dev
