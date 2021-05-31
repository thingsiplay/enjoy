# enjoy for RetroArch

Play any game ROM with associated emulator in *RetroArch*

- **Author**: Tuncay D.
- **License**: [MIT License](LICENSE)
- **Source**: [Github](https://github.com/thingsiplay/enjoy)

![enjoy](./img/enjoy_logo.svg "enjoy")

## Introduction

**enjoy** is a wrapper around *RetroArch* on Linux to help running emulator
cores through the commandline.  This functionality can be used to configure
your system to launch games from *RetroArch* directly within your file manager
by (double) clicking the ROM files too.  The main magic comes from a user
configuration file with rules and aliases to extensions and emulator core
names.

### Example

```bash
$ enjoy -w '~/roms/snes/Super Mario World (U) [!].smc'
```

### Features

- run *RetroArch* games directly from your console or file manager
- combine it with other programs output like `grep`, `ls` or `dmenu` to play a
  game from a list of ROM files
- fast startup
- easy to configure in a single INI file format

### Quick Start

1. Install and setup *RetroArch* first, if not done already:
   [RetroArch](https://www.retroarch.com/)
2. Download **enjoy**:
   [Releases](https://github.com/thingsiplay/enjoy/releases)
3. Optionally, install it in a directory within *$PATH*.  The default
   *"install.sh"* script that comes with the downloadable distribution archive
   does it.
4. Execute `enjoy --open-config` to open the default configuration file or
   create one at *"~/.config/enjoy/default.ini"*.  You have to setup first,
   explained in [Configuration File](#configuration_file).
5. Optionally, register the specific ROM file extensions (in example *".smc"*)
   to this program.
6. Enjoy.

## Installation

### Download the binary

Download the archive *"enjoy_0.1.0.tar.gz"* from
[Releases](https://github.com/thingsiplay/enjoy/releases) and unpack it.  The
provided installation script *"install.sh"* is optional and will copy the
binary to the directory determined with `systemd-path user-binaries`.  Which
resolves in my case to *"/home/tuncay/.local/bin"*.  It will also copy an
example configuration file to *"~/.config/enjoy/default.ini"*, if not already
exists.

### File extension registering (optional)

There are multiple ways of achieving this, depending on the environment you
are.  A simple way is to right click on the ROM file of your choice and select
something like "Open with..." and choose the program **enjoy** or type the path
where it is installed to.  The installation path of **enjoy** can vary, but in
my case it is in *"/home/tuncay/.local/bin"*.  Also there should be a tickbox
available to make it the default program for this file extension.  From now on
you should be able to play the games with this extension from your file manager
by double clicking it.

### Build from source

```bash
git clone https://github.com/thingsiplay/enjoy
```

To build from source code, the *Rust* tools are required, as it is written in
*Rust*.  A simple `cargo build --release` is enough to create the binary.

The optional "*Makefile*" is just a convenient way to compile and build the
distribution archives and uses additional programs, such as

- `cargo clippy`
- `upx` (disabled at default)
- `strip`
- `pandoc`

A resulting archive file *"enjoy_0.1.0-tar.gz"* should be created in a new
subdirectory *"dist"*.

## Configuration File

At default the configuration file at *"~/.config/enjoy/default.ini"* is loaded
up and the specified rules define what emulator core to choose.  It is an INI
format and easy to understand.  There are 4 different kind of categories.

### [options] - Main Options

```ini
[options]
retroarch = /usr/bin/retroarch
libretro_directory = /home/user/.config/retroarch/cores
fullscreen = 1
```

These are for the most part the same options from the commandline interface.
To see the full list of options, run the program with `enjoy -h` for a short
overview or `enjoy --help` for a longer description.

There are a few differences when using the same option in the configuration
file.  In example the the option the program accepts multiple games, but in the
config file can only be one game at `game = path` specified.  Flags like
`--fullscreen` needs to be specified with a value of `1` or `true` in file.

Some options are not available in the file, such as `--noconfig` or `--config`.

### [cores] - Core Rules

```ini
[cores]
snes = snes9x
mdwide = /home/user/.config/retroarch/cores/genesis_plus_gx_wide_libretro.so
gb = sameboy_libretro.so
gbc = sameboy
```

These are the user defined aliases which points to the *"libretro"* cores in
*"RetroArch"*.  On the left side `snes` is the custom name and on the right
side of equal sign is the path or filename of a *"libretro"* core, in example
`snes9x`.  The part *"_libretro.so"* is optional and will be added
automatically.  If the path is relative, then the *"libretro-directory"* is
looked up.  If the directory is not specified, then the program tries to optain
it from *"RetroArch"* itself.

Core Rules can be used from commandline option as `--core snes` or are looked
up from the Extension Rules and Directory Rules.

### [.ext1 .ext2] - Extension Rules

```ini
[.smc .sfc]
core = snes

[.gb]
core = gb

[.nes]
core = gb
libretro = mesen_libretro.so

[.n64 .z64]
libretro = mupen64plus_next
```

These rules specify which emulator to run, based on the file extension of the
ROM file.  The section name contains the extensions to associate with.  There
can be multiple extensions per rule, separated by space.  Each of them has to
start with a dot, to be recognized as an Extension Rule.

It can have a rule for a core alias, which would be looked up in section
`[cores]` described above. Alternatively a path to an emulator executable can
be specified directly.  If both are present, then the `libretro` rule has
higher priority.

### [/path/to/directory] - Directory Rules

```ini
[/home/user/roms/psx/]
core = psx
libretro = mednafen_psx_hw_libretro.so
```

Any game in a folder or subfolder specified in these rules are associated
automatically with the given core and have higher priorities over Extension
Rules.  These Directory Rules are useful for extensions that are used with
different kind of ROM formats, like ".iso" or ".chd" in example.

## Known Bugs, Limitations and Quirks

- Some options from `retroarch` itself  are not supported yet.
- When executing the emulator command, the GUI of RetroArch is still loaded up
  and will not close automatically.  It is recommnded to use 2 times `ESC` to
  end the play session, if the GUI is not needed.
- The Flatpak or Snap version of *"RetroArch"* might not work with this
  program.


