use std::path::PathBuf;

use structopt::StructOpt;

/// Play any game ROM with associated emulator in `"RetroArch"`.
///
/// This program is a launcher to run the `"RetroArch"` commandline program to play the games
/// directly.  A path to a game rom must be specified through an argument or stdin pipe. Depending
/// on the file extension and the rules in the user configuration file, a predefined emulator core
/// will be selected.  Use option `-h` for compact summary or `--help` for longer description of
/// options.  <https://github.com/thingsiplay/enjoy/>
///
/// Examples:
///
/// $ enjoy -F1 '~/roms/snes/Super Mario World (U) [!].smc'
///
/// $ ls -1 ./snes/* | enjoy --filter '[!]' --core snesgood --which
#[derive(Debug, StructOpt)]
pub(crate) struct Opt {
    /// Path to ROM file to play
    ///
    /// If multiple files are specified, then the first entry is picked.  Each line from stdin is
    /// added as a game entry too.  Globbing and wildcards are not supported and should be resolved
    /// by the shell.  Relative paths and the tilde are supported and expanded.
    ///
    /// Example: "~/roms/snes/Super Mario World (U) [\!].smc"
    #[structopt(parse(from_os_str))]
    pub(crate) games: Vec<PathBuf>,

    /// Path to the user settings
    ///
    /// This programs own configuration file in the INI format contains extension rules and core
    /// naming aliases to actual libretro files.  The commandline arguments with the same name as
    /// the options in this file have higher priority over the settings.  Whenever the program
    /// runs, these settings are loaded and looked up.
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
    /// The executable name or path to the RetroArch commandline application to run.  This can be a
    /// fullpath or a filename only, in which case it will be searched in the system `$PATH`.
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
    /// the config folder of `RetroArch` itself.  By default these locations in this order are
    /// looked up: `$XDG_CONFIG_HOME/retroarch/retroarch.cfg`,
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
    /// The explicit filename of the emulator in `RetroArch`.  This option overwrites any
    /// previously setting or rule and forces to launch the specified emulator.  It can be a
    /// fullpath or a filename only, in which case it will be looked up in the directory specified
    /// at `libretro-directory`.  The filename part `_libretro.so` is optional and will be added
    /// automatically when needed.  `snes9x` could be expanded into
    /// `/home/user/.config/retroarch/cores/snes9x_libretro.so` as an example.
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

    /// Directory of libretro core files for `RetroArch`
    ///
    /// The installation directory of the libretro emulator cores.  This will be used to lookup if
    /// the `libretro` setting or option is a relative filename only, and ignored otherwise.  At
    /// default this value will be looked up and read from the `RetroArch` base configuration file
    /// `retroarch.cfg`.
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
    /// `[cores]` as `name=libretro`.
    ///
    /// Example: "snes"
    #[structopt(short = "C", long, value_name = "ALIAS", display_order = 3)]
    pub(crate) core: Option<String>,

    /// Apply simple wildcard to filter list of games
    ///
    /// Removes all games from the list, which do not match the `pattern`.  The wildcard
    /// functionality is limited and only the star `*` and questionmark `?` are supported.  The
    /// comparison is always case insensitive.  It will compare the base filename portion of the
    /// ROM path to the pattern, ignoring it's parent directory and filename extension parts.  At
    /// default a star is added in front and end of pattern automatically when comparing.  This
    /// option is useful if more than one game entry is given to the program.
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
    /// Prevents from running another `retroarch` process, if one is already active.  In this case
    /// the command to play will not execute.
    #[structopt(short = "1", long, display_order = 2)]
    pub(crate) highlander: bool,

    /// Show user settings
    ///
    /// Opens the current config INI file of this program with it's associated default application.
    #[structopt(short = "o", long, display_order = 3)]
    pub(crate) open_config: bool,

    /// Ignore user settings
    ///
    /// The config INI file of this program will be ignored and not loaded up.  The entire
    /// application relies on commandline options and environmental variables.  Which means that
    /// any predefined rules and aliases are ignored.
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
    /// still simulated, just before the point of running the emulator.
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
