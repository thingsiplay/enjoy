use std::path::PathBuf;

use clap::value_parser;
use clap::Parser;

/// Play any game ROM with associated emulator in `RetroArch`.
///
/// `enjoy` is a launcher to run games from `RetroArch` without using the GUI.  It is a wrapper
/// around the `retroarch` commandline application.  The main functionality comes from a user
/// configuration file with rules to associate file extensions and emulator cores.  When running
/// `enjoy` only the path to a ROM file is required, either through arguments or the stdin.  Use
/// option `-h` for compact summary or `--help` for longer description of options.
/// <https://github.com/thingsiplay/enjoy/>
///
/// Examples:
///
/// $ enjoy '~/roms/snes/Super Mario World (U) [!].smc'
///
/// $ enjoy 'Super Mario World (U) [!].smc' -w
///
/// $ ls -1 ./snes/* | enjoy --filter '[!]' --core snes --which --highlander
///
/// $ ls -1 $(readlink -f ~/roms/gb)/* | enjoy -xWn
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[command(version, author, after_help = "https://github.com/thingsiplay/enjoy")]
pub struct Opt {
    /// Path to ROM file
    ///
    /// If multiple files are specified, then the first entry is picked when starting emulator.
    /// Each line from stdin is added as a game entry too.  Globbing and wildcards are not
    /// supported and should be resolved by the shell.  Relative paths and the tilde are supported
    /// and expanded.
    ///
    /// Example: "~/roms/snes/Super Mario World (U) [\!].smc"
    #[arg(value_parser=value_parser!(PathBuf))]
    pub games: Vec<PathBuf>,

    /// Bypass additional arguments to `retroarch`
    ///
    /// Everything after a standalone double dash `--` is redirected to `retroarch` without
    /// interpreting the arguments.  This can be used for options that are unsupported or not
    /// implemented in `enjoy` yet.  They are and not expanded or error checked and the user has to
    /// ensure correctness.
    ///
    /// Example: "-- --set-shader ''"
    #[arg(last = true)]
    pub retroarch_arguments: Vec<String>,

    /// Path to the user settings
    ///
    /// This programs own configuration file in INI format.  It contains all user defined rules to
    /// associated extensions and core name aliases.  Any option specified at commandline have
    /// higher priority over the individual settings in this file.
    ///
    /// Example: "/home/user/.config/enjoy/alternative.ini"
    #[arg(
        short = 'c',
        long,
        value_name = "FILE",
        display_order = 1,
        default_value = "~/.config/enjoy/default.ini"
    )]
    pub config: PathBuf,

    /// Open user settings
    ///
    /// Opens the user config INI file with it's associated default application and exit.
    #[arg(short = 'O', long, display_order = 1)]
    pub open_config: bool,

    /// Print path of user settings
    ///
    /// Prints path of the user config INI file to stdout and exit.
    #[arg(short = 'o', long, display_order = 1)]
    pub config_path: bool,

    /// Path or name of `RetroArch` command
    ///
    /// The executable name or path to the `RetroArch` commandline application to run.  If this is a
    /// filename without directory part, then the systems `$PATH` is searched.
    ///
    /// Example: "/usr/bin/retroarch" [default: retroarch]
    #[arg(short = 'A', long, value_name = "APP", display_order = 7)]
    pub retroarch: Option<PathBuf>,

    /// Path to `RetroArch` base configuration
    ///
    /// The `retroarch.cfg` base configuration file of `RetroArch` itself.  Usually it is found in
    /// the config folder of `RetroArch` itself.  By default these locations are looked up in this
    /// particular order: `$XDG_CONFIG_HOME/retroarch/retroarch.cfg`,
    /// `$HOME/.config/retroarch/retroarch.cfg`, `$HOME/.retroarch.cfg`.
    ///
    /// Example: "/home/user/.config/retroarch/retroarch.cfg"
    #[arg(short = 'B', long, value_name = "FILE", display_order = 7)]
    pub retroarch_config: Option<PathBuf>,

    /// Force specific libretro core by filename
    ///
    /// The explicit filename of the emulator in `RetroArch`.  This option overwrites any previous
    /// setting or rule and forces to launch the specified emulator.  This can be a fullpath or
    /// filename only.  If this is filename only, then the directory part is looked up from
    /// `libretro-directory`.  The filename part `_libretro.so` is optional and will be added
    /// automatically.  As an example `snes9x` could be expanded into
    /// `/home/user/.config/retroarch/cores/snes9x_libretro.so`.
    ///
    /// Example: "snes9x"
    #[arg(
        short = 'L',
        long,
        value_name = "FILE",
        display_order = 5,
        conflicts_with = "core"
    )]
    pub libretro: Option<PathBuf>,

    /// Directory of libretro core files
    ///
    /// The installation directory of libretro cores.  It is looked up whenever the `libretro` path
    /// is a relative filename.  At default this directory is extracted from `RetroArch` base
    /// configuration file `retroarch.cfg`.
    ///
    /// Example: "/home/user/.config/retroarch/cores"
    #[arg(short = 'D', long, value_name = "DIR", display_order = 6)]
    pub libretro_directory: Option<PathBuf>,

    /// Force specific libretro core by user defined alias
    ///
    /// A custom identificator specified in the user configuration INI file.  The alias will be
    /// looked up and resolved into a real `libretro` path.  These are specified under the section
    /// `[cores]` as `alias=libretro_path`.
    ///
    /// Example: "snes"
    #[arg(short = 'C', long, value_name = "ALIAS", display_order = 4)]
    pub core: Option<String>,

    /// Apply simple wildcard to filter list of games
    ///
    /// Removes all games from the list, which do not match the `pattern`.  The wildcard
    /// functionality is limited and only the star `*` and questionmark `?` are supported.  The
    /// comparison is always case insensitive.  It will compare the base filename portion of the
    /// ROM path to the pattern, ignoring it's parent directory and filename extension.  At default
    /// a star is added in front and end of pattern automatically when comparing.  It is useful if
    /// more than one game entry is given to the program.  This option can be specified multiple
    /// times.  All of them have to match.
    ///
    /// Example: "mario*[\!]"
    #[arg(short = 'f', long, value_name = "PATTERN", display_order = 2)]
    pub filter: Option<Vec<String>>,

    /// Strict mode for filter
    ///
    /// Turns the option `--filter` to be more strict when comparing filenames.  It makes it case
    /// sensitive and a word will match the beginning to end of filename, no longer are stars "*"
    /// surrounding the search pattern added to match any part.
    #[arg(short = 's', long, display_order = 2)]
    pub strict: bool,

    /// Print selected game ROM
    ///
    /// Writes the full filepath of the selected game to stdout.
    #[arg(short = 'w', long, display_order = 1)]
    pub which: bool,

    /// Print `RetroArch` commandline
    ///
    /// Writes full command with all arguments used to run `RetroArch` to stdout. Has higher priority
    /// than option --which.
    #[arg(short = 'W', long, display_order = 1)]
    pub which_command: bool,

    /// Print all core names
    ///
    /// Lists all core names on the left side of the user configuration under section "\[cores\]".
    /// Will output matching cores to the libretro core that would be used with the game.  Without
    /// a game, all cores are listed.
    #[arg(short = 'n', long, display_order = 3)]
    pub list_cores: bool,

    /// Force fullscreen mode
    ///
    /// Runs the emulator and `RetroArch` UI in fullscreen, regardless of any other setting.
    #[arg(short = 'F', long, display_order = 3)]
    pub fullscreen: bool,

    /// Follow symbolic links
    ///
    /// When expanding paths, this option will ensure the Rom file path will resolve symbolic links
    /// too. Otherwise the program will only see and run the command with the symlink Rom file
    /// instead, which can have a different name and location than its target.
    #[arg(short = 'l', long, display_order = 3)]
    pub resolve: bool,

    /// There Can Only Be One!
    ///
    /// Prevents running another `retroarch` process, if one is already active.  In this case the
    /// final command of the emulator will not execute.
    #[arg(short = '1', long, display_order = 3)]
    pub highlander: bool,

    /// Ignore user settings
    ///
    /// The config INI file of this program will be ignored and not loaded up.  The entire
    /// application relies on commandline options and environmental variables.  Therefore any
    /// predefined rules and aliases from that file are ignored.
    #[arg(
        short = 'i',
        long,
        display_order = 8,
        conflicts_with_all = &["config", "open_config", "core"]
    )]
    pub noconfig: bool,

    /// Do not run `RetroArch`
    ///
    /// The `retroarch` run command to play ROMs will not be executed.  Internally the process is
    /// still simulated, up until to the point of running the emulator.  If a game ROM is not
    /// found, then the simulation will continue to allow execution of other options.
    #[arg(short = 'x', long, display_order = 8)]
    pub norun: bool,

    /// Dismiss reading from stdin
    ///
    /// Ignores the `stdin` and do not test or read any data from it.  Normally the program will
    /// look and read all lines from `stdin` as additional game entries.  This option will disable
    /// that.
    #[arg(short = 'z', long, display_order = 8)]
    pub nostdin: bool,
}
