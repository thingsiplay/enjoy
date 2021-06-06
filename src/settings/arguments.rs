use std::path::PathBuf;

use structopt::StructOpt;

/// Play any game ROM with associated emulator in `"RetroArch"`.
///
/// `enjoy` is a launcher to run games from `RetroArch` without using the GUI.  It is a wrapper
/// around the `retroarch` commandline application.  The main functionality comes from a user
/// configuration file with rules to associate file extensions and emulator cores.  When running
/// **enjoy** only the path to a ROM file is required, either through arguments or the stdin.  Use
/// option `-h` for compact summary or `--help` for longer description of options.
/// <https://github.com/thingsiplay/enjoy/>
///
/// Examples:
///
/// $ enjoy '~/roms/snes/Super Mario World (U) [!].smc'
///
/// $ ls -1 ./snes/* | enjoy --filter '[!]' --core snes --which --highlander
#[derive(Debug, StructOpt)]
pub(crate) struct Opt {
    /// Path to ROM file
    ///
    /// If multiple files are specified, then the first entry is picked when starting emulator.
    /// Each line from stdin is added as a game entry too.  Globbing and wildcards are not
    /// supported and should be resolved by the shell.  Relative paths and the tilde are supported
    /// and expanded.
    ///
    /// Example: "~/roms/snes/Super Mario World (U) [\!].smc"
    #[structopt(parse(from_os_str))]
    pub(crate) games: Vec<PathBuf>,

    /// Path to the user settings
    ///
    /// This programs own configuration file in INI format.  It contains all user defined rules to
    /// associated extensions and core name aliases.  Any option specified at commandline have
    /// higher priority over the individual settings in this file.
    ///
    /// Example: "/home/user/.config/enjoy/alternative.ini"
    #[structopt(
        short = "c",
        long,
        parse(from_os_str),
        value_name = "FILE",
        display_order = 1,
        default_value = "~/.config/enjoy/default.ini"
    )]
    pub(crate) config: PathBuf,

    /// Path or name of `RetroArch` command
    ///
    /// The executable name or path to the RetroArch commandline application to run.  If this is a
    /// filename without directory part, then the systems `$PATH` is searched.
    ///
    /// Example: "/usr/bin/retroarch" [default: retroarch]
    #[structopt(
        short = "A",
        long,
        parse(from_os_str),
        value_name = "APP",
        display_order = 4
    )]
    pub(crate) retroarch: Option<PathBuf>,

    /// Path to `RetroArch` base configuration
    ///
    /// The `retroarch.cfg` base configuration file of `RetroArch` itself.  Usually it is found in
    /// the config folder of `RetroArch` itself.  By default these locations are looked up in this
    /// particular order: `$XDG_CONFIG_HOME/retroarch/retroarch.cfg`,
    /// `$HOME/.config/retroarch/retroarch.cfg`, `$HOME/.retroarch.cfg`.
    ///
    /// Example: "/home/user/.config/retroarch/retroarch.cfg"
    #[structopt(
        short = "B",
        long,
        parse(from_os_str),
        value_name = "FILE",
        display_order = 4
    )]
    pub(crate) retroarch_config: Option<PathBuf>,

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
    #[structopt(
        short = "L",
        long,
        parse(from_os_str),
        value_name = "FILE",
        display_order = 3,
        conflicts_with = "core"
    )]
    pub(crate) libretro: Option<PathBuf>,

    /// Directory of libretro core files
    ///
    /// The installation directory of libretro cores.  It is looked up whenever the `libretro` path
    /// is a relative filename.  At default this directory is extracted from `RetroArch` base
    /// configuration file `retroarch.cfg`.
    ///
    /// Example: "/home/user/.config/retroarch/cores"
    #[structopt(
        short = "D",
        long,
        parse(from_os_str),
        value_name = "DIR",
        display_order = 3
    )]
    pub(crate) libretro_directory: Option<PathBuf>,

    /// Force specific libretro core by user defined alias
    ///
    /// A custom identificator specified in the user configuration INI file.  The alias will be
    /// looked up and resolved into a real `libretro` path.  These are specified under the section
    /// `[cores]` as `alias=libretro_path`.
    ///
    /// Example: "snes"
    #[structopt(short = "C", long, value_name = "ALIAS", display_order = 3)]
    pub(crate) core: Option<String>,

    /// Apply simple wildcard to filter list of games
    ///
    /// Removes all games from the list, which do not match the `pattern`.  The wildcard
    /// functionality is limited and only the star `*` and questionmark `?` are supported.  The
    /// comparison is always case insensitive.  It will compare the base filename portion of the
    /// ROM path to the pattern, ignoring it's parent directory and filename extension.  At default
    /// a star is added in front and end of pattern automatically when comparing.  This option is
    /// useful if more than one game entry is given to the program.
    ///
    /// Example: "mario*[\!]"
    #[structopt(short = "f", long, value_name = "PATTERN", display_order = 2)]
    pub(crate) filter: Option<String>,

    /// Print selected game ROM
    ///
    /// Writes the full filepath of the selected game to stdout.
    #[structopt(short = "w", long, display_order = 1)]
    pub(crate) which: bool,

    /// Force fullscreen mode
    ///
    /// Runs the emulator and `RetroArch` UI in fullscreen, regardless of any other setting.
    #[structopt(short = "F", long, display_order = 2)]
    pub(crate) fullscreen: bool,

    /// There Can Only Be One!
    ///
    /// Prevents running another `retroarch` process, if one is already active.  In this case the
    /// final command of the emulator will not execute.
    #[structopt(short = "1", long, display_order = 2)]
    pub(crate) highlander: bool,

    /// Show user settings
    ///
    /// Opens the user config INI file with it's associated default application.
    #[structopt(short = "o", long, display_order = 3)]
    pub(crate) open_config: bool,

    /// Ignore user settings
    ///
    /// The config INI file of this program will be ignored and not loaded up.  The entire
    /// application relies on commandline options and environmental variables.  Therefore any
    /// predefined rules and aliases from that file are ignored.
    #[structopt(
        short = "i",
        long,
        display_order = 4,
        conflicts_with_all = &["config", "open-config", "core"]
    )]
    pub(crate) noconfig: bool,

    /// Do not run `RetroArch`
    ///
    /// The `retroarch` run command to play ROMs will not be executed.  Internally the process is
    /// still simulated, up until to the point of running the emulator.
    #[structopt(short = "x", long, display_order = 4)]
    pub(crate) norun: bool,

    /// Dismiss reading from stdin
    ///
    /// Ignores the `stdin` and do not test or read any data from it.  Normally the program will
    /// look and read all lines from `stdin` as additional game entries.  This option will disable
    /// that.
    #[structopt(short = "z", long, display_order = 4)]
    pub(crate) nostdin: bool,
}
