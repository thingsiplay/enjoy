# enjoy for RetroArch

Play any game ROM with associated emulator in *RetroArch* on Linux

- **Author**: Tuncay D.
- **License**: [MIT License](LICENSE)
- **Documentation**: [Wiki](https://github.com/thingsiplay/enjoy/wiki)
- **Source**: [Github](https://github.com/thingsiplay/enjoy)
- **Download**: [Github](https://github.com/thingsiplay/enjoy/releases)
- **Rust Package Registry**: [Crates](https://crates.io/crates/enjoy/)

![enjoy](./img/enjoy_logo.svg "enjoy")

## Introduction

**enjoy** is a wrapper around "RetroArch" on Linux to help running emulator
cores on the commandline.  A user configuration file can be setup, including
rules and core aliases pointing to file extensions and emulator paths.  Each
time the program runs, it will lookup these settings to determine the correct
core for each given extension.  It can be even used to launch games directly
within your favorite file manager by double clicking the ROM file, if file
extension is registered to open **enjoy** as the default program.

Use option `-h` for short help and `--help` for detailed help.

### Example

```bash
$ enjoy '~/roms/snes/Super Mario World (U) [!].smc'

$ ls -1 $(readlink -f ~/Emulatoren/games/gb)/* | enjoy --filter 'mario' -xWn

$ find . -maxdepth 2 | fzf | enjoy -xWn
```

Depending on your shell, you might need to escape the `!` in example. When
multiple ROMs are given, then the first one will be loaded. There are many
options available, including filtering such a game list or output the entire
command used to run RetroArch.

### Features

- run *RetroArch* games directly from terminal or filemanager
- combine from output like `grep`, `ls` or `dmenu` to select a file
- fast startup
- easy to configure

### Quick Start

Install and setup *RetroArch* first, if not done already:
[RetroArch](https://www.retroarch.com/)

#### Install **enjoy**, if you have cargo installed ...

- Build and install from [crates.io](https://crates.io/crates/enjoy/) with
   `cargo install enjoy`.

#### ... or get binary manually from Github

- Download **enjoy** binary from
  [Releases](https://github.com/thingsiplay/enjoy/releases) and unpack it.
- Optionally, copy the file "enjoy" to a directory within *$PATH* and set the
  executable bit.  The *"install.sh"* script does that for you, if you want.
  Read a more detailed description about the installation process in the Wiki:
  [Installation](https://github.com/thingsiplay/enjoy/wiki/Installation)

#### Configure the user settings

- Execute `enjoy --open-config` to open the default configuration file or
  create one at "~/.config/enjoy/default.ini" if it does not exist.  Read more
  about the configuration at
  [User Configuration File](User-Configuration-File).
- Optionally, register **enjoy** as the default program to the specific ROM
  file extensions (in example *".smc"*).

Enjoy.

## User Configuration File

Read a more detailed description about the file in the Wiki:
[User Configuration File](https://github.com/thingsiplay/enjoy/wiki/User-Configuration-File)

The default configuration file at "~/.config/enjoy/default.ini" will be
automatically loaded up each time `enjoy` is executed.  Example:

```ini
[options]
retroarch = /usr/bin/retroarch

[cores]
snes = snes9x
gb gbc = sameboy_libretro.so

[.smc .sfc]
core = snes

[.gb]
core = gb

[/home/user/roms/psx*]
libretro = mednafen_psx_hw
```

There are 4 different categories of sections.

- `[options]` - *Main Options*:  These are the same options found in the
  commandline interface of the program.  Use `enjoy -h` for short overview or
  `enjoy --help` for a longer description of all possible options.

- `[cores]` - *Core Rules*:  Custom alias to any "libretro" core from
  "RetroArch".  On the left side is the name of the core.  It can be a list of
  names separated by space.  On the right side is the filename or entire path
  of an emulator core.  If filename has no directory part, then the core will
  be searched in "libretro-directory" (which is the path where your cores are
  configured and installed in RetroArch).  The part "\_libretro.so" in the
  filename is optional.

- `[.ext1 .ext2]` - *Extension Rules*:  When a game ROM is loaded up, it's file
  extension is compared to these rules.  Each extension rule consists of a
  single or group of space separated extensions.  Each section name has to
  start with a dot to be recognized as an "Extension Rule".  It can contain a
  `core` option, which will be looked up in section `[cores]` to determine
  libretro path.  Or it can have `libretro` option, which has highest priority
  and points directly to an libretro path.

- `[/path/to/directory]` - *Directory Rules*:  Any section with a slash in the
  name is a "Directory Rule".  When a game ROM is loaded up, it's directory
  part excluding the filename is compared against theses rules.  Basic
  wildcards are supported too.  The star `*`, to match none or any number of
  characters and question mark, to match a single character.  If a match is
  found, then it's associated `core` or `libretro` option will be looked up.

## Known Bugs, Limitations and Quirks

- Not all commandline options from `retroarch` main program are supported.  As
  a workaround arguments can be directly passed over to `retroarch` with the
  option `--` on the commandline or `retroarch_arguments =` option in the
  configuration file of **enjoy**.

- The RetroArch GUI will be loaded up each time a game runs.  At default the
  `ESC`-key can be pressed two times in row to quickly end the current play
  session, closing RetroArch and the GUI.

- The Flatpak or Snap version of *"RetroArch"* might not work with this
  program, as it was not tested (reports are welcome).
