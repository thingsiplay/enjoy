# enjoy for RetroArch

Play any game ROM with associated emulator in *RetroArch*

- **Author**: Tuncay D.
- **License**: [MIT License](LICENSE)
- **Documentation**: [Wiki](https://github.com/thingsiplay/enjoy/wiki)
- **Source**: [Github](https://github.com/thingsiplay/enjoy)
- **Download**: [Github](https://github.com/thingsiplay/enjoy/releases)
- **Rust Package Registry**: [Crates](https://crates.io/crates/enjoy/)

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
$ enjoy '~/roms/snes/Super Mario World (U) [!].smc'
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
   explained in [User Configuration File](#user_configuration_file).
5. Optionally, register the specific ROM file extensions (in example *".smc"*)
   to this program.
6. Enjoy.

## Installation

Read a more detailed description about the installation in the Wiki:
[Installation](https://github.com/thingsiplay/enjoy/wiki/Installation)

Download and unpack the binary from [Releases](https://github.com/thingsiplay/enjoy/releases)
Use provided "install.sh" script is optional and will copy the binary to the
user binaries folder.  Alongside it an example configuration file is copied to
"~/.config/enjoy/default.ini".

## User Configuration File

Read a more detailed description about the file in the Wiki:
[User Configuration File](https://github.com/thingsiplay/enjoy/wiki/User-Configuration-File)

The default configuration file at "~/.config/enjoy/default.ini" will be
automatically loaded up each time `enjoy` is executed.  It is in an easy to
understand and editable INI format.  Here a quick overview:

```ini
[options]
retroarch = /usr/bin/retroarch

[cores]
snes = snes9x
gb = sameboy_libretro.so

[.smc .sfc]
core = snes

[/home/user/roms/psx/]
core = psx
```

There are 4 different categories of sections.

- `[options]` - Main Options:  These are the same options found in the
  commandline interface of the program.  Use `enjoy -h` for short overview or
  `enjoy --help` for a longer description of all possible options.
- `[cores]` - Core Rules:  Custom alias to any "libretro" core from
  "RetroArch".  On the left side is the name of the core and on the right side
  the path or filename of an emulator core.  If the filename has no directory
  part, then it will be searched in the "libretro-directory".  The part
  "_libretro.so" in the filename part is optional.
- `[.ext1 .ext2]` - Extension Rules:  When a game ROM is loaded up, it's
  extension is compared if one of these matches.  Each extension rule consists
  of a single or a group of space separated extensions.  Each has to start with
  a dot in their section names.  Their rules can include a `core` rule, which
  will be looked up at section `[cores]`.  Or it can directly have `libretro`
  rule, which is a path to an emulator.
- `[/path/to/directory]` - Directory Rules:  Any section with a slash in the
  name is a directory rule.  In this case the folder in which the loaded up
  game ROM is compared, instead it's extension.  If the game is in one of these
  folders, then these rules kick in.  These are the same `core` and `libretro`
  rules.

Directory Rules should be used sparingly.

## Known Bugs, Limitations and Quirks

- Some options from `retroarch` itself  are not supported yet.
- When executing the emulator command, the GUI of RetroArch is still loaded up
  and will not close automatically.  It is recommnded to use 2 times `ESC` to
  end the play session, if the GUI is not needed.
- The Flatpak or Snap version of *"RetroArch"* might not work with this
  program.


