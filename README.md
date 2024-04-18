# enjoy for RetroArch

Play any game ROM with associated emulator in _RetroArch_ on Linux

- **Author**: Tuncay D.
- **License**: [MIT License](LICENSE)
- **Documentation**: [Wiki](https://github.com/thingsiplay/enjoy/wiki)
- **Source**: [Github](https://github.com/thingsiplay/enjoy)
- **Download**: [Github](https://github.com/thingsiplay/enjoy/releases)
- **Rust Package Registry**: [Crates](https://crates.io/crates/enjoy/)

![enjoy](./img/enjoy_logo.svg "logo")

## Introduction

**enjoy** is a blazingly fast wrapper around "RetroArch" on Linux, to help
running emulator cores on the commandline. A user configuration file can be
setup, including rules and core aliases pointing to file extensions and
emulator paths.

Each time it runs, settings from user configuration or commandline arguments
will be looked up to determine the correct libretro core path for each given
extension (such as `.smc` would be associated with `snes9x` or any other core).
Or launch games directly within your favorite graphical filemanager, by double
clicking the Rom file. For this `enjoy` needs to be configured as the default
"open with"-application for the specified file extension in your file manager.

Use commandline option `-h` for short help and `--help` for detailed help.

### Few Usage Examples

The lines starting with a hash symbol `#` are comments explaining the purpose
of the following commands.

```bash
# Core for this file extension will be looked up from config file.
enjoy '~/Emulation/Roms/snes/Super Mario World (U) [!].smc'

# Arguments after standalone `--` will be given over to RetroArch as they are.
enjoy '~/Emulation/Roms/snes/Super Mario World (U) [!].smc' -- --set-shader ''

# Do not run, but output matching core alias and RetroArch command instead.
enjoy '~/Emulation/Roms/snes/Super Mario World (U) [!].smc' -xnW

# Specified core alias will be looked up from config file.
enjoy --core psx 'Gran Turismo 2 - Epic Turismo 2 (v1.3.1).chd'

# Or choose a core directly by it's RetroArch file name of the core.
enjoy -L mednafen_psx_hw 'Gran Turismo 2 - Epic Turismo 2 (v1.3.1).chd'

# Combine list of Roms from other programs output. First entry from a list is
# selected to play.
find ~/Emulation/Roms/gb | enjoy

# Use the filter to narrow down the input list even further.
find ~/Emulation/Roms/gb | enjoy --filter 'mario'

# Create custom menus with a fuzzy finder and choose the game interactively.
find . -maxdepth 2 | fzf | enjoy -w
```

Depending on your shell, you might need to escape the "`!`" in example. When
a list of multiple ROMs are given as input, then the first one will be loaded.

### Features

- run _RetroArch_ games directly from terminal or filemanager
- combine from output like `grep`, `find`, `fzf` or `dmenu` to select a file
- fast startup
- easy to configure
- lot of enjoyable moments

### Quick Start

Install and setup _RetroArch_ first, if not done already:
[RetroArch](https://www.retroarch.com/)

#### If you have cargo

- Build and install from [crates.io](https://crates.io/crates/enjoy/) with:

  ```bash
  cargo install enjoy
  ```

#### or get binary manually from GitHub

- Download from [Releases](https://github.com/thingsiplay/enjoy/releases) and
  unpack it.
- Optionally, copy the file "enjoy" to a directory within _$PATH_ and set the
  executable bit. Read a more detailed description about the installation process
  in the Wiki:
  [Installation](https://github.com/thingsiplay/enjoy/wiki/Installation)

#### Configure the user settings

- Execute `enjoy --open-config` to open the default configuration file or
  create one if it does not exist at: "~/.config/enjoy/default.ini" . Have a look
  in "example-config.ini" to see how the file is structured. Read more about the
  configuration at [User Configuration File](User-Configuration-File).
- Optionally, register **enjoy** as the default application to open with. Right
  click on the Rom file, open its properties and set the default program for this
  file extension (in example on ".smc" files) to `enjoy` . Next time you double
  click the file would automatically open RetroArch with this game and the
  correct core. No need for the terminal.

Enjoy.

## User Configuration File

Read a more detailed description about the file in the Wiki:
[User Configuration File](https://github.com/thingsiplay/enjoy/wiki/User-Configuration-File)

The default configuration file at "~/.config/enjoy/default.ini" will be
automatically loaded up each time `enjoy` is executed. Example:

```ini
[options]
retroarch = /usr/bin/retroarch
fullscreen = 1

[cores]
snes = snes9x
gb gbc = sameboy_libretro.so

[.smc .sfc]
core = snes

[.gb]
core = gb

[/home/user/Roms/psx*]
libretro = mednafen_psx_hw
```

There are 4 different categories of sections.

- `[options]` - _Main Options_: These are the same options found in the
  commandline interface of the program. Use `enjoy -h` for short overview or
  `enjoy --help` for a longer description of all possible options.

- `[cores]` - _Core Rules_: Custom alias to any "libretro" core from
  "RetroArch". On the left side is the name of the core. It can be a list of
  names (such as `snes` or `gb gbc`) separated by space. On the right side is the
  filename or entire path (such as `snes9x` or `snes9x_libretro.so` or
  `/home/tuncay/.config/retroarch/cores/snes9x_libretro.so`) of an emulator core.
  If filename has no directory part, then the core will be searched in
  "libretro-directory" (which is the path where your cores are configured and
  installed in RetroArch). The part "\_libretro.so" in the filename is optional.

- `[.ext1 .ext2]` - _Extension Rules_: When a game ROM is loaded up, it's file
  extension is compared to these rules. Each extension rule consists of a single
  or group (such as `[.gb]` or `[.smc .sfc]`) of space separated extensions. Each
  section name has to start with a dot to be recognized as an "Extension Rule".
  The body of the rule can contain a `core` alias setting (such as `core =
snes`), which will be looked up in section `[cores]` to determine libretro name
  and path. Or specify the name of the core with `libretro` setting (such as
  `libretro = mednafen_psx_hw`), which has highest priority and points directly
  to an libretro path.

- `[/path/to/directory]` - _Directory Rules_: Any section with a slash in the
  name (such as `/home/user/Roms/psx*`) is a "Directory Rule". When a game ROM is
  loaded up (such as `/home/tuncay/Emulation/Roms/psx/Metal Gear Solid (USA)
(Disc 1) (v1.1).chd`), it's directory part excluding the filename (in this
  example `/home/tuncay/Emulation/Roms/psx`) is compared against theses rules. If
  the directory portion of the game path matches to any Directory Rule, then it
  takes priority over any other Extension Rules setting.

  This is especially useful for generic extensions like `.chd`, so they can be
  identified by folder instead. Basic wildcards are supported too. The star
  `*`, to match none or any number of characters and question mark, to match a
  single character. If a match is found, then it's associated `core` or
  `libretro` option will be looked up.

## Known Bugs, Limitations and Quirks

- Not all commandline options from `retroarch` main program are supported. As
  a workaround arguments can be directly passed over to `retroarch` with the
  option `--` on the commandline or `retroarch_arguments =` option in the
  configuration file of **enjoy**.

- The RetroArch GUI will be loaded up each time a game runs. At default the
  `ESC`-key can be pressed two times in row to quickly end the current play
  session, closing RetroArch and the GUI.

- The Flatpak or Snap version of _"RetroArch"_ might not work with this
  program, as it was not tested (reports are welcome).
