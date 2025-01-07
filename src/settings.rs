mod arguments;
mod file;
mod inoutput;
mod retroarch;

use arguments::Opt;

use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use clap::Parser;
use configparser::ini;
use indexmap::map::IndexMap;
use wildmatch::WildMatch;

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

/// The final `process::Command` to execute and run `retroarch`.  It bundles related information
/// such as paths and the `output` from stdout.  The additional path data should be manually set
/// when building the main `cmdline`, to stay in sync.  The `output` must be set manually after
/// executing `cmdline` process.
#[derive(Debug)]
pub struct RunCommand {
    pub cmdline: Command,
    pub game: PathBuf,
    pub libretro: PathBuf,
    pub output: Option<Output>,
}

/// Configuration of the main program.  The intended use case is to create multiple `Settings` data
/// from various places like commandline arguments or user configuration file.  Then all those
/// `Settings` data should be merged into a single one, which will be used as the source when
/// finally building the `RunCommand`.  Which is then used to execute `retroarch` program itself.
#[derive(Debug)]
pub struct Settings {
    games: Vec<PathBuf>,
    retroarch_arguments: Vec<String>,
    config: Option<PathBuf>,
    retroarch: Option<PathBuf>,
    retroarch_config: Option<PathBuf>,
    libretro: Option<PathBuf>,
    libretro_directory: Option<PathBuf>,
    core: Option<String>,
    filter: Option<Vec<String>>,
    strict: Option<bool>,
    which: Option<bool>,
    which_command: Option<bool>,
    list_cores: Option<bool>,
    fullscreen: Option<bool>,
    resolve: Option<bool>,
    highlander: Option<bool>,
    open_config: Option<bool>,
    config_path: Option<bool>,
    noconfig: Option<bool>,
    norun: Option<bool>,
    nostdin: Option<bool>,
    cores_rules: Option<IndexMap<String, PathBuf>>,
    extension_rules: Option<IndexMap<String, PathBuf>>,
    directory_rules: Option<IndexMap<String, PathBuf>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            games: vec![],
            retroarch_arguments: vec![],
            config: None,
            retroarch: None,
            retroarch_config: None,
            libretro: None,
            libretro_directory: None,
            core: None,
            filter: None,
            strict: None,
            which: None,
            which_command: None,
            list_cores: None,
            fullscreen: None,
            resolve: None,
            highlander: None,
            open_config: None,
            config_path: None,
            noconfig: None,
            norun: None,
            nostdin: None,
            cores_rules: None,
            extension_rules: None,
            directory_rules: None,
        }
    }

    /// Read each line from stdin stream and convert it to paths.  Create a new struct with games
    /// out of it.
    pub fn new_from_stdin(nostdin: bool) -> Result<Self> {
        let mut settings: Self = Self::new();

        if !nostdin {
            let list = inoutput::list_from_stdin()?;
            settings.games = list.iter().map(PathBuf::from).collect();
        }

        Ok(settings)
    }

    /// Create a new Settings struct with a few default data.
    pub fn new_from_defaults() -> Self {
        let mut settings: Self = Self::new();

        settings.retroarch = Some(PathBuf::from("retroarch"));

        settings
    }

    /// Parse own commandline arguments and create a new Settings struct out of it.
    pub fn new_from_cmdline(options: Option<Vec<String>>) -> Self {
        let mut settings: Self = Self::new();

        let args: Opt = match options {
            Some(opt) => Opt::parse_from(opt.iter()),
            None => Opt::parse(),
        };

        // default_value
        // Take them, as they have a default value anyway.
        settings.config = Some(args.config);

        // list
        // Take it, as it is always a list.
        settings.games = args.games;
        settings.retroarch_arguments = args.retroarch_arguments;

        // Option
        // Take them, as they are optional anyway.
        settings.retroarch = args.retroarch;
        settings.retroarch_config = args.retroarch_config;
        settings.libretro = args.libretro;
        settings.libretro_directory = args.libretro_directory;
        settings.core = args.core;
        settings.filter = args.filter;

        // bool
        // Only set it to `true`, if the option is found in arguments.
        if args.strict {
            settings.strict = Some(true);
        }
        if args.which {
            settings.which = Some(true);
        }
        if args.which_command {
            settings.which_command = Some(true);
        }
        if args.list_cores {
            settings.list_cores = Some(true);
        }
        if args.fullscreen {
            settings.fullscreen = Some(true);
        }
        if args.resolve {
            settings.resolve = Some(true);
        }
        if args.highlander {
            settings.highlander = Some(true);
        }
        if args.open_config {
            settings.open_config = Some(true);
        }
        if args.config_path {
            settings.config_path = Some(true);
        }
        if args.noconfig {
            settings.noconfig = Some(true);
        }
        if args.norun {
            settings.norun = Some(true);
        }
        if args.nostdin {
            settings.nostdin = Some(true);
        }

        settings
    }

    /// Parse `retroarch.cfg` the own configuration file of `RetroArch` itself and create a new
    /// `Settings` struct out of it.
    pub fn new_from_retroarch_config(file: Option<&PathBuf>) -> Result<Self> {
        let mut settings: Self = Self::new();

        // If no file was given, then search at `RetroArch` default locations for the file `retroarch.cfg`.
        settings.retroarch_config = match file {
            Some(p) => file::to_fullpath(p, false),
            None => retroarch::search_default_config(),
        };

        // The list of key names to search and extract.  Ignore all other.
        let mut keys_to_get: HashSet<String> = HashSet::new();
        keys_to_get.insert("libretro_directory".to_string());

        let retroarch_config_map =
            retroarch::parse_retroarch_config(settings.retroarch_config.as_ref(), &keys_to_get)?;

        // Extract values.
        if let Some(value) = retroarch_config_map.get("libretro_directory") {
            settings.libretro_directory = Some(PathBuf::from(value));
        }

        Ok(settings)
    }

    /// Parse programs user configuration INI file and create a new `Settings` struct out of it.
    ///
    /// Example structure:
    ///
    /// ```ini
    /// [options]
    /// retroarch = /usr/bin/retroarch
    /// libretro-directory = /home/user/.config/retroarch/cores
    /// fullscreen = 1
    /// highlander = 1
    ///
    /// [cores]
    /// snes = snes9x
    /// mdwide = genesis_plus_gx_wide
    /// gb gbc = sameboy
    ///
    /// [~/roms/genesis_wide*]
    /// core = mdwide
    ///
    /// [.smc, .sfc]
    /// core = snes
    ///
    /// [.md, .gen]
    /// libretro = genesis_plus_gx
    /// ```
    pub fn new_from_config(file: Option<&PathBuf>) -> Result<Self> {
        let mut settings: Self = Self::new();

        let path: PathBuf = match file {
            Some(p) => p.clone(),
            None => return Ok(settings),
        };
        // Extend the path and resolve to fullpath.
        match file::to_fullpath(&path, false) {
            Some(fullpath) => settings.config = Some(fullpath),
            None => {
                return Err(format!("User config ini file not found: {}", path.display()).into());
            }
        };

        let mut ini: ini::Ini = ini::Ini::new_cs();
        ini.load(file::to_str(settings.config.as_ref()))
            .expect("Error in loading configuration file.");

        let section_names: Vec<String> = ini.sections();

        // [options]
        // retroarch = /usr/bin/retroarch
        Self::read_config_options(&mut settings, &ini, &section_names)?;

        // [cores]
        // snes = snes9x
        let cores_rules: IndexMap<String, PathBuf> = Self::read_config_cores_rules(&ini);
        if !cores_rules.is_empty() {
            settings.cores_rules.replace(cores_rules);
        }

        // [.smc .sfc]
        // core = snes
        // libretro = snes9x
        let extension_rules: IndexMap<String, PathBuf> =
            Self::read_config_extension_rules(settings.cores_rules.as_ref(), &ini, &section_names);
        if !extension_rules.is_empty() {
            settings.extension_rules.replace(extension_rules);
        }

        // [/home/user/roms/genesis_wide]
        // core = mdwide
        let directory_rules: IndexMap<String, PathBuf> =
            Self::read_config_directory_rules(settings.cores_rules.as_ref(), &ini, &section_names);
        if !directory_rules.is_empty() {
            settings.directory_rules.replace(directory_rules);
        }

        Ok(settings)
    }

    /// Read the keys in section `[options]` from ini and update corresponding application
    /// `Settings` struct directly.  Update only from existing keys.  As a sidenote, these keys
    /// represent the same options from the commandline arguments.  Notably the option `game` in
    /// the config file can be only a single path, unlike the arguments.  And any arguments
    /// including a dash inside the argument name (in example `--retroarch-config`) will in the
    /// file use underscore instead (in example `retroarch_config`).
    ///
    /// Read all key/value pairs from section `[options]` in the INI.  Only existing keys are
    /// updated on the corresponding `Settings` fields, without affecting the others.  These are
    /// the same options found in the programs commandlline arguments (use `enjoy --help` to find
    /// out more).  There are only a couple of differences to it:
    ///
    /// 1. There is only one `game` entry instead of a list.
    /// 2. Some options about the configuration file itself are not present here, because they are
    ///    evaluated before loading the INI: `--config`, `--open-config` or `--noconfig`.
    /// 3. Flags in the commandline can be used here with a value of `1` or `true` to indicate they
    ///    are active.  This is needed, because each key or option in the INI file has a value,
    ///    which is not the case on the commandline.  An example would be `--norun`, which is then
    ///    translated into INI key `norun = 1`.
    ///
    /// ```ini
    /// [options]
    /// retroarch = /usr/bin/retroarch
    /// ```
    fn read_config_options(
        settings: &mut Self,
        ini: &ini::Ini,
        section_names: &[String],
    ) -> Result<()> {
        if section_names.contains(&String::from("options")) {
            if let Some(value) = ini.get("options", "game") {
                settings.games.push(PathBuf::from(value));
            }
            if let Some(value) = ini.get("options", "retroarch_arguments") {
                settings.retroarch_arguments = shlex::split(&value).unwrap_or_default();
            }
            if let Some(value) = ini.get("options", "retroarch") {
                settings.retroarch = Some(PathBuf::from(value));
            }
            if let Some(value) = ini.get("options", "retroarch_config") {
                settings.retroarch_config = Some(PathBuf::from(value));
            }
            if let Some(value) = ini.get("options", "libretro") {
                settings.libretro = Some(PathBuf::from(value));
            }
            if let Some(value) = ini.get("options", "libretro_directory") {
                settings.libretro_directory = Some(PathBuf::from(value));
            }
            if let Some(value) = ini.get("options", "core") {
                settings.core = Some(value);
            }
            if let Some(value) = ini.get("options", "filter") {
                settings.filter = Some(vec![value]);
            }
            if let Some(value) = ini.getboolcoerce("options", "strict")? {
                settings.strict = Some(value);
            }
            if let Some(value) = ini.getboolcoerce("options", "which")? {
                settings.which = Some(value);
            }
            if let Some(value) = ini.getboolcoerce("options", "which_command")? {
                settings.which = Some(value);
            }
            if let Some(value) = ini.getboolcoerce("options", "list_cores")? {
                settings.list_cores = Some(value);
            }
            if let Some(value) = ini.getboolcoerce("options", "fullscreen")? {
                settings.fullscreen = Some(value);
            }
            if let Some(value) = ini.getboolcoerce("options", "resolve")? {
                settings.resolve = Some(value);
            }
            if let Some(value) = ini.getboolcoerce("options", "highlander")? {
                settings.highlander = Some(value);
            }
            if let Some(value) = ini.getboolcoerce("options", "norun")? {
                settings.norun = Some(value);
            }
            if let Some(value) = ini.getboolcoerce("options", "nostdin")? {
                settings.nostdin = Some(value);
            }
        }

        Ok(())
    }

    /// Extract user defined alias mappings for `core` names and their associated `path` in section
    /// `[cores]`.
    ///
    /// ```ini
    /// [cores]
    /// snes = snes9x
    /// ```
    fn read_config_cores_rules(ini: &ini::Ini) -> IndexMap<String, PathBuf> {
        let mut cores_rules: IndexMap<String, PathBuf> = IndexMap::new();

        if let Some(cores) = ini.get_map().unwrap_or_default().get("cores") {
            // Get valid entries only and convert to `(String, String)`.
            for (core_alias, libretro_path) in cores
                .iter()
                .filter(|(_, v)| !v.as_ref().unwrap_or(&String::new()).is_empty())
                .map(|(k, v)| (k.to_string(), v.as_ref().unwrap()))
            {
                for alias in core_alias.split_whitespace() {
                    cores_rules.insert(alias.to_string(), PathBuf::from(libretro_path));
                }
            }
        }

        cores_rules
    }

    /// Read in all rules for the extensions from ini.  `extension_rules` start with a dot in their
    /// section name like `[.smc .sfc]`.  Multiple extensions can be space separated per rule.  The
    /// leading dot will be removed.  Any `core` rule will be resolved to a `libretro` path by
    /// looking up corresponding alias in `cores_rules`.  An existing `libretro` rule have higher
    /// priority over `core` rule.
    ///
    /// ```ini
    /// [.smc .sfc]
    /// core = snes
    /// ```
    fn read_config_extension_rules(
        cores_rules: Option<&IndexMap<String, PathBuf>>,
        ini: &ini::Ini,
        section_names: &[String],
    ) -> IndexMap<String, PathBuf> {
        let mut extension_rules: IndexMap<String, PathBuf> = IndexMap::new();

        for pattern_group in section_names.iter().filter(|e| e.starts_with('.')) {
            // [.smc .sfc]
            // Iterate over each extension and remove their leading dot.
            for ext_pattern in pattern_group
                .split_whitespace()
                .map(|e| e.split_at(1).1.to_string())
            {
                // libretro = snes9x
                // Take libretro path directly.
                if let Some(path) = ini.get(pattern_group, "libretro") {
                    extension_rules.insert(ext_pattern, PathBuf::from(path));
                }
                // core = snes
                // Lookup matching libretro path from rules.
                else if let Some(core_alias) = ini.get(pattern_group, "core") {
                    // [cores]
                    // snes = snes9x
                    if let Some(path) = cores_rules.as_ref().unwrap().get(&core_alias) {
                        extension_rules.insert(ext_pattern, PathBuf::from(path));
                    }
                }
            }
        }

        extension_rules
    }

    /// Read in all rules for the directories from ini.  `directory_rules` include a slash
    /// somewhere in their section name like `[/emulators/roms/psx]`.  The starting tilde will be
    /// expanded to users home directory.  Any `core` rule will be resolved to a `libretro` path by
    /// looking up corresponding alias in `cores_rules`.  An existing `libretro` rule have higher
    /// priority over `core` rule.
    ///
    /// ```ini
    /// [/home/user/roms/genesis_wide]
    /// core = mdwide
    /// ```
    fn read_config_directory_rules(
        cores_rules: Option<&IndexMap<String, PathBuf>>,
        ini: &ini::Ini,
        section_names: &[String],
    ) -> IndexMap<String, PathBuf> {
        let mut directory_rules: IndexMap<String, PathBuf> = IndexMap::new();

        // Find all sections which include a slash, to indicate its a directory.  Any tilde will
        // will be expanded to users home directory.  Create a tuple group for each directory, with
        // its original path and the expanded one.  The original is needed later to lookup again
        // and the expanded will be assign to the final returning `directory_rules`.
        let dir_pattern: Vec<(String, String)> = section_names
            .iter()
            .filter(|dir| dir.contains('/'))
            .map(|dir| (dir.to_string(), shellexpand::tilde(dir).to_string()))
            .collect();

        // [/home/user/roms/genesis_wide*]
        for (original, expanded) in dir_pattern {
            // libretro = snes9x
            // Take libretro path directly.
            if let Some(path) = ini.get(&original, "libretro") {
                directory_rules.insert(expanded, PathBuf::from(path));
            }
            // core = snes
            // Lookup matching libretro path from rules.
            else if let Some(core_alias) = ini.get(&original, "core") {
                // [cores]
                // snes = snes9x
                if let Some(path) = cores_rules.as_ref().unwrap().get(&core_alias) {
                    directory_rules.insert(expanded, PathBuf::from(path));
                }
            }
        }

        directory_rules
    }

    /// Merge current `Settings` with a new one.  Overwrite values only, if the new value is
    /// `Some`. The `games` key is different, as the new list in `games` will be prepended to
    /// current existing list.
    pub fn update_from(&mut self, overwrite: Self) {
        if !overwrite.games.is_empty() {
            if self.games.is_empty() {
                self.games = overwrite.games;
            } else {
                let mut combined: Vec<PathBuf> = overwrite.games;
                combined.append(&mut self.games);
                self.games = combined;
            }
        }

        if !overwrite.retroarch_arguments.is_empty() {
            if self.retroarch_arguments.is_empty() {
                self.retroarch_arguments = overwrite.retroarch_arguments;
            } else {
                self.retroarch_arguments
                    .append(&mut overwrite.retroarch_arguments.clone());
            }
        }

        if overwrite.config.is_some() {
            self.config = overwrite.config;
        }
        if overwrite.retroarch.is_some() {
            self.retroarch = overwrite.retroarch;
        }
        if overwrite.retroarch_config.is_some() {
            self.retroarch_config = overwrite.retroarch_config;
        }
        if overwrite.libretro.is_some() {
            self.libretro = overwrite.libretro;
        }
        if overwrite.libretro_directory.is_some() {
            self.libretro_directory = overwrite.libretro_directory;
        }
        if overwrite.core.is_some() {
            self.core = overwrite.core;
        }
        if overwrite.filter.is_some() {
            self.filter = overwrite.filter;
        }
        if overwrite.strict.is_some() {
            self.strict = overwrite.strict;
        }
        if overwrite.which.is_some() {
            self.which = overwrite.which;
        }
        if overwrite.which_command.is_some() {
            self.which_command = overwrite.which_command;
        }
        if overwrite.list_cores.is_some() {
            self.list_cores = overwrite.list_cores;
        }
        if overwrite.fullscreen.is_some() {
            self.fullscreen = overwrite.fullscreen;
        }
        if overwrite.resolve.is_some() {
            self.resolve = overwrite.resolve;
        }
        if overwrite.highlander.is_some() {
            self.highlander = overwrite.highlander;
        }
        if overwrite.open_config.is_some() {
            self.open_config = overwrite.open_config;
        }
        if overwrite.config_path.is_some() {
            self.config_path = overwrite.config_path;
        }
        if overwrite.noconfig.is_some() {
            self.noconfig = overwrite.noconfig;
        }
        if overwrite.norun.is_some() {
            self.norun = overwrite.norun;
        }
        if overwrite.nostdin.is_some() {
            self.nostdin = overwrite.nostdin;
        }

        // Currenty, the IndexMap rules are just replaced.  In future they will be possibly
        // extended instead.
        if overwrite.cores_rules.is_some() {
            self.cores_rules = overwrite.cores_rules;
        }
        if overwrite.extension_rules.is_some() {
            self.extension_rules = overwrite.extension_rules;
        }
        if overwrite.directory_rules.is_some() {
            self.directory_rules = overwrite.directory_rules;
        }
    }

    /// Update current Settings from new Settings.  Replace the content only, if the old value is
    /// `None`.  Only a few keys are affected, currently `retroarch`, `retroarch_config`,
    /// `libretro` and `libretro_directory`.
    pub fn update_defaults_from(&mut self, overwrite: Self) {
        if self.retroarch.is_none() {
            self.retroarch = overwrite.retroarch;
        }
        if self.retroarch_config.is_none() {
            self.retroarch_config = overwrite.retroarch_config;
        }
        if self.libretro.is_none() {
            self.libretro = overwrite.libretro;
        }
        if self.libretro_directory.is_none() {
            self.libretro_directory = overwrite.libretro_directory;
        }
    }

    /// Build up the final `RetroArch` run command from the current Settings.  This is the command
    /// and its options that is used when executing `retroarch` commandline application.  It will
    /// be wrapped up in a separate `RunCommand` struct, which itself includes the commandline to
    /// execute and a few more data.
    pub fn build_command(&self) -> Result<RunCommand, String> {
        // `--retroarch`
        let mut command: Command = Command::new(file::to_str(self.retroarch.as_ref()));

        // `game`
        // Get first entry of all games in the list, make it a full path and check if file exists.
        let game: Option<PathBuf> = match self.select_game() {
            Some(selected) => {
                let path = file::to_fullpath(&selected, self.resolve.unwrap_or_default());
                match path {
                    Some(ref p) => command.arg(p),
                    None => {
                        if self.is_norun() {
                            command.arg(&selected)
                        } else {
                            let message = format!("game file not found: {}", selected.display());
                            return Err(message);
                        }
                    }
                };

                if path.is_some() {
                    path
                } else if self.is_norun() {
                    Some(selected)
                } else {
                    None
                }
            }
            None => {
                if self.norun.unwrap_or(false) {
                    command.arg("");
                    Some(PathBuf::from(String::new()))
                } else {
                    return Err("No matching game available".into());
                }
            }
        };

        // `--libretro`
        let mut libretro: Option<PathBuf> = self.libretro.clone();

        // `libretro` have higher priority over `core`, if present.  Otherwise lookup `core`, if
        // available.
        if libretro.is_none() {
            // `--core`
            if let Some(core) = &self.core {
                match &self.cores_rules {
                    Some(rules) => libretro = rules.get(core).cloned(),
                    None => return Err("No core rules found in `[cores]`.".into()),
                };
            }

            // Lookup and resolve from `[/directory]` rules
            if libretro.is_none() && self.directory_rules.is_some() {
                libretro = self.libretro_from_dir(
                    game.as_ref()
                        .expect("game required when building libretro path from directory rules."),
                );
            };
            // Lookup and resolve from `[.ext]` rules
            if libretro.is_none() && self.extension_rules.is_some() {
                libretro = self.libretro_from_ext(
                    game.as_ref()
                        .expect("game required when building libretro path from extension rules."),
                );
            };
        }

        // At this point, the `libretro` path should be available, either given directly or by
        // resolving rules from `core`.
        if libretro.is_none() {
            return Err("Path to `libretro` not set.".into());
        }

        // Combine `--libretro_directory` and `--libretro`
        // If the `libretro` itself is a relative path, then it will be combined with the given
        // directory.  Otherwise the directory is ignored, as a fullpath of `libretro` takes
        // precedence.
        match retroarch::libretro_fullpath(
            self.libretro_directory.clone(),
            libretro.clone(),
            "_libretro.so",
        ) {
            Some(fullpath) => {
                libretro = Some(fullpath.clone());
                command.arg("--libretro");
                command.arg(fullpath);
            }
            None => return Err("No matching libretro core found".into()),
        };

        // `--retroarch-config`
        if let Some(file) = &self.retroarch_config {
            command.arg("--config");
            command.arg(file);
        }

        // `--fullscreen`
        if self.fullscreen.unwrap_or(false) {
            command.arg("--fullscreen");
        }

        // `--`
        if !self.retroarch_arguments.is_empty() {
            command.args(self.retroarch_arguments.iter());
        }

        // Use `run.cmdline` to get the full command with all options to be executed.  `output`
        // needs to be updated manually, by catching the output when running the `cmdline`.
        let run = RunCommand {
            cmdline: command,
            game: game.unwrap_or_default(),
            libretro: libretro.unwrap_or_default(),
            output: None,
        };

        Ok(run)
    }

    /// Find core matching the libretro to list of cores.
    pub fn find_core_match(&self, libretro: &Path) -> Vec<String> {
        let mut core_match: Vec<String> = vec![];

        if let Some(rules) = &self.cores_rules {
            let libretro_string = libretro
                .to_path_buf()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            for (core, path) in rules {
                let path_string = path.file_stem().unwrap().to_string_lossy().to_string();
                if path_string.trim_end_matches("_libretro")
                    == libretro_string.trim_end_matches("_libretro")
                {
                    core_match.push(core.to_string());
                }
            }
        }

        core_match
    }

    /// Extract extension from game path and lookup the corresponding extension rule in current
    /// settings to get the `libretro` path.
    fn libretro_from_ext(&self, game: &Path) -> Option<PathBuf> {
        if let Some(game_ext) = game.extension() {
            if let Some(extension_rules) = &self.extension_rules.as_ref() {
                if let Some(libretro) = extension_rules.get(
                    game_ext
                        .to_str()
                        .expect("Non UTF-8 character in extension."),
                ) {
                    return Some(libretro.clone());
                }
            }
        }

        None
    }

    /// Extract parent folder from game path and lookup the corresponding directory rule in current
    /// settings to get the `libretro` path.
    fn libretro_from_dir(&self, game: &Path) -> Option<PathBuf> {
        if let Some(game_parent) = game.parent() {
            if let Some(directory_rules) = &self.directory_rules.as_ref() {
                if let Some(rule) = directory_rules.iter().find(|(directory, _)| {
                    WildMatch::new(&file::trim_last_slash((*directory).to_string())).matches(
                        game_parent
                            .as_os_str()
                            .to_str()
                            .expect("game folder as valid string"),
                    )
                }) {
                    return Some(rule.1.clone());
                }
            }
        }

        None
    }

    /// Extract the first game entry from current Settings `games` list.  If any filter is
    /// available, then apply it before extraction.  The comparison is always in lowercase.
    /// Supported special characters are only the star "*", for matching anything and questionmark
    /// "?", for matching a single character.  The filter will be enclosed by stars automatically.
    fn select_game(&self) -> Option<PathBuf> {
        match &self.filter {
            Some(filter) => {
                let pattern_wildmatch = self.pattern_list_wildmatch(filter);

                for game in &self.games {
                    let gstring: String = self.to_lowercase(
                        &game
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap_or_default()
                            .to_owned(),
                    );

                    if pattern_wildmatch
                        .iter()
                        .all(|pattern| pattern.matches(&gstring))
                    {
                        return Some(game.clone());
                    }
                }

                None
            }
            None => self.games.first().cloned(),
        }
    }

    /// Simply convert a String to lowercase if `strict` mode is off.
    fn to_lowercase(&self, text: &String) -> String {
        if self.is_strict() {
            text.to_string()
        } else {
            text.to_lowercase()
        }
    }

    /// Build the list of patterns by wildcard filter.
    fn pattern_list_wildmatch(&self, filter: &[String]) -> Vec<WildMatch> {
        filter
            .iter()
            .map(|pattern| self.wildmatch_pattern(pattern))
            .collect()
    }

    /// Build the wildmatch pattern based on `strict` mode of `filter` option.
    fn wildmatch_pattern(&self, pattern: &str) -> WildMatch {
        if self.is_strict() {
            WildMatch::new(pattern)
        } else {
            WildMatch::new(&format!("*{}*", pattern.to_lowercase()))
        }
    }

    /// Opens the current `config` file with the associated default application.
    pub fn open_config(&self) -> Result<bool> {
        if self.open_config.unwrap_or(false) {
            let config_path: &PathBuf = self
                .config
                .as_ref()
                .expect("Path to config ini file required.");

            if let Some(path) = file::to_fullpath(config_path, false) {
                file::open_with_default(&path)?;
            }

            return Ok(true);
        }

        Ok(false)
    }

    /// Get the user configuration INI file path from `config` option in current Settings.  Default
    /// to `None`, if option `noconfig` is active.
    #[must_use]
    pub fn get_config(&self) -> Option<&PathBuf> {
        if self.noconfig.unwrap_or(false) {
            None
        } else {
            self.config.as_ref()
        }
    }

    /// Get the `RetroArchs` own `retroarch.cfg` configuration file path from current Settings.
    #[must_use]
    pub const fn get_retroarch_config(&self) -> Option<&PathBuf> {
        self.retroarch_config.as_ref()
    }

    /// Check if current Settings has a `game` path entry available.
    #[must_use]
    pub fn is_game_available(&self) -> bool {
        !self.games.is_empty()
    }

    /// Check if current Settings has a `libretro` path to a file available.
    #[must_use]
    pub fn is_libretro_path_available(&self) -> bool {
        match &self.libretro {
            Some(path) => path.has_root(),
            None => return false,
        };

        self.libretro_directory.is_some()
    }

    /// Check if the `strict` option is set, so the `filter` command will go into strict mode.
    pub fn is_strict(&self) -> bool {
        self.strict.unwrap_or(false)
    }

    /// Check if the `stdin` stream should be ignored.
    #[must_use]
    pub fn is_nostdin(&self) -> bool {
        self.nostdin.unwrap_or(false)
    }

    /// Check if the `norun` option is set, so `RetroArch` command will not be executed.
    pub fn is_norun(&self) -> bool {
        self.norun.unwrap_or(false)
    }

    /// Print the given `path`, if current Settings include the option `which`.
    pub fn print_which(&self, path: &PathBuf) {
        if self.which.unwrap_or(false) {
            inoutput::print_path(Some(path));
        }
    }

    /// Print path of user settings file defined in `config`.
    pub fn print_config(&self) -> bool {
        if self.config_path.unwrap_or(false) {
            inoutput::print_fullpath(self.config.as_ref());
            return true;
        }

        false
    }

    /// Check if option to print entire command is set.
    pub fn is_which_command(&self) -> bool {
        self.which_command.unwrap_or(false)
    }

    /// Check if option to print cores is set.
    pub fn is_list_cores(&self) -> bool {
        self.list_cores.unwrap_or(false)
    }

    /// Print all name of cores defined in the section \[cores\] in the config file.
    pub fn print_cores(&self) {
        if let Some(rules) = self.cores_rules.as_ref() {
            let mut keys: Vec<String> = rules.clone().into_keys().collect();
            keys.sort_unstable();
            for core in keys {
                println!("{core}");
            }
        }
    }

    /// Check if an instance of `RetroArch` is already running, if the single instance mode
    /// `highlander` is active.  Otherwise its always `false`.
    #[must_use]
    pub fn there_can_only_be_one(&self) -> bool {
        self.highlander.unwrap_or(false) && retroarch::is_running("retroarch", true)
    }

    /// Execute the given `Command` to run the program with its arguments and return its `output`.
    /// Do not execute it, if the option `norun` is active.
    pub fn run(&self, command: &mut Command) -> Option<Output> {
        if self.norun.unwrap_or(false) {
            None
        } else {
            let output: Output = command.output().expect("Error! Could not run RetroArch.");
            // if output.status.to_string() != *"exit code: 0" {
            if output.status.to_string() != *"exit status: 0" {
                eprintln!("Could not run RetroArch. {}", output.status);
            }

            Some(output)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use configparser::ini;
    use indexmap::map::IndexMap;

    type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    // Untested:
    //  - Settings::new_from_stdin()
    //  - Settings::new_from_retroarch_config()
    //  - Settings::new_from_config()
    //  - Settings::update_defaults_from()
    //  - Settings::build_command()
    //  - Settings::open_config()
    //  - Settings::get_config()
    //  - Settings::get_retroarch_config()
    //  - Settings::is_game_available()
    //  - Settings::is_nostdin()
    //  - Settings::print_which()
    //  - Settings::is_which_command()
    //  - Settings::list_cores()
    //  - Settings::print_cores()
    //  - Settings::there_can_only_be_one()
    //  - Settings::run()

    #[test]
    fn new_from_defaults_retroarch() -> Result<()> {
        let settings = super::Settings {
            games: vec![],
            retroarch_arguments: vec![],
            config: None,
            retroarch: Some(PathBuf::from("retroarch")),
            retroarch_config: None,
            libretro: None,
            libretro_directory: None,
            core: None,
            filter: None,
            strict: None,
            which: None,
            which_command: None,
            list_cores: None,
            fullscreen: None,
            resolve: None,
            highlander: None,
            open_config: None,
            config_path: None,
            noconfig: None,
            norun: None,
            nostdin: None,
            cores_rules: None,
            extension_rules: None,
            directory_rules: None,
        };

        let defaults = super::Settings::new_from_defaults();

        assert_eq!(settings.retroarch, defaults.retroarch);

        Ok(())
    }

    #[test]
    fn new_from_cmdline_default_config() -> Result<()> {
        let options: Vec<String> = vec!["enjoy".to_string()];
        let test_config = Some(PathBuf::from("~/.config/enjoy/default.ini"));
        let args = super::Settings::new_from_cmdline(Some(options));

        assert_eq!(test_config, args.config);
        assert_eq!(None, args.norun);

        Ok(())
    }

    //NOTE: An empty "" argument does not work anymore and is replaced by space " ".
    #[test]
    fn new_from_cmdline_emptygame_then_retroarch() -> Result<()> {
        let options: Vec<String> = vec![
            "enjoy".to_string(),
            " ".to_string(),
            "--retroarch".to_string(),
            "/usr/bin/retroarch".to_string(),
        ];
        let args = super::Settings::new_from_cmdline(Some(options));

        assert_eq!(Some(PathBuf::from("/usr/bin/retroarch")), args.retroarch);
        assert_eq!(vec![PathBuf::from(" ")], args.games);

        Ok(())
    }

    //NOTE: An empty "" argument does not work anymore and is replaced by space " ".
    #[test]
    fn new_from_cmdline_game() -> Result<()> {
        let options: Vec<String> = vec![
            "enjoy".to_string(),
            "mario.smc".to_string(),
            " ".to_string(),
        ];
        let test_games: Vec<PathBuf> = vec![PathBuf::from("mario.smc"), PathBuf::from(" ")];
        let args = super::Settings::new_from_cmdline(Some(options));

        assert_eq!(test_games, args.games);

        Ok(())
    }

    fn test_ini_template() -> ini::Ini {
        let content = String::from(
            "
            [options]
            retroarch = /usr/bin/retroarch
            retroarch_arguments = --verbose --set-shader \"\"
            which = 0
            norun = true
            libretro_directory=

            [retroarch]
            which = 1
            doesnotexist
            doesexist = 0

            [cores]
            snes = snes9x
            md = genesis_plus_gx_libretro.so
            mdwide = genesis_plus_gx_wide
            gb gbc = sameboy


            [/bin*]
            core = md
            libretro = mednafen_psx_hw

            [path_without_slash]
            core = snes

            [.smc .sfc]
            core = snes

            [.mdwide]
            core = mdwide
            ",
        );
        let mut ini: ini::Ini = ini::Ini::new_cs();

        ini.read(content).unwrap();

        ini
    }

    // This is the content of `[cores]` in `test_ini_template()`.  Use this template, to test if
    // the INI content is handled as expected.
    fn test_ini_cores_rules_template() -> IndexMap<String, PathBuf> {
        let mut cores_rules: IndexMap<String, PathBuf> = IndexMap::new();

        cores_rules.insert("snes".to_string(), PathBuf::from("snes9x"));
        cores_rules.insert(
            "md".to_string(),
            PathBuf::from("genesis_plus_gx_libretro.so"),
        );
        cores_rules.insert("mdwide".to_string(), PathBuf::from("genesis_plus_gx_wide"));

        cores_rules
    }

    #[test]
    fn read_config_options_retroarch_arguments() -> Result<()> {
        let mut settings = super::Settings::new();
        let ini = test_ini_template();

        super::Settings::read_config_options(&mut settings, &ini, &["options".to_string()])?;

        assert_eq!(
            vec!["--verbose", "--set-shader", ""],
            settings.retroarch_arguments
        );

        Ok(())
    }

    #[test]
    fn read_config_options_path() -> Result<()> {
        let mut settings = super::Settings::new();
        let ini = test_ini_template();

        super::Settings::read_config_options(&mut settings, &ini, &["options".to_string()])?;

        assert_eq!(
            Some(PathBuf::from("/usr/bin/retroarch")),
            settings.retroarch
        );
        assert_eq!(Some(PathBuf::from("")), settings.libretro_directory);
        assert_eq!(None, settings.retroarch_config);

        Ok(())
    }

    #[test]
    fn read_config_options_bool() -> Result<()> {
        let mut settings = super::Settings::new();
        let ini = test_ini_template();

        super::Settings::read_config_options(&mut settings, &ini, &["options".to_string()])?;

        assert_eq!(Some(false), settings.which);
        assert_eq!(Some(true), settings.norun);

        Ok(())
    }

    #[test]
    fn read_config_cores_rules() -> Result<()> {
        let ini = test_ini_template();

        let rules = super::Settings::read_config_cores_rules(&ini);

        assert_eq!(Some(&PathBuf::from("snes9x")), rules.get("snes"));
        assert_eq!(
            Some(&PathBuf::from("genesis_plus_gx_libretro.so")),
            rules.get("md")
        );
        assert_eq!(None, rules.get("retroarch"));

        Ok(())
    }

    #[test]
    fn read_config_extension_rules() {
        let ini = test_ini_template();

        let ext_rules = super::Settings::read_config_extension_rules(
            Some(test_ini_cores_rules_template()).as_ref(),
            &ini,
            &ini.sections(),
        );

        assert_eq!(Some(&PathBuf::from("snes9x")), ext_rules.get("sfc"));
        assert_eq!(
            Some(&PathBuf::from("genesis_plus_gx_wide")),
            ext_rules.get("mdwide")
        );
        assert_eq!(None, ext_rules.get(""));
    }

    #[test]
    fn read_config_directory_rules() {
        let ini = test_ini_template();

        let dir_rules = super::Settings::read_config_directory_rules(
            Some(test_ini_cores_rules_template()).as_ref(),
            &ini,
            &ini.sections(),
        );

        assert_eq!(
            Some(&PathBuf::from("mednafen_psx_hw")),
            dir_rules.get("/bin*")
        );
        assert_ne!(Some(&PathBuf::from("md")), dir_rules.get("/bin"));
        assert_eq!(None, dir_rules.get("path_without_slash"));
    }

    #[test]
    fn libretro_from_ext() {
        let mut ext_rules: IndexMap<String, PathBuf> = IndexMap::new();
        ext_rules.insert("sfc".to_string(), PathBuf::from("mesen-s"));
        ext_rules.insert("smc".to_string(), PathBuf::from("mesen-s"));
        ext_rules.insert("gb".to_string(), PathBuf::from("sameboy"));
        ext_rules.insert("gbc".to_string(), PathBuf::from("sameboy"));

        let settings = super::Settings {
            games: vec![],
            retroarch_arguments: vec![],
            config: None,
            retroarch: Some(PathBuf::from("retroarch")),
            retroarch_config: None,
            libretro: None,
            libretro_directory: None,
            core: None,
            filter: Some(vec!["[!]".to_string()]),
            strict: None,
            which: None,
            which_command: None,
            list_cores: None,
            fullscreen: None,
            resolve: None,
            highlander: Some(true),
            open_config: None,
            config_path: None,
            noconfig: None,
            norun: Some(true),
            nostdin: None,
            cores_rules: None,
            extension_rules: Some(ext_rules),
            directory_rules: None,
        };

        assert_eq!(
            None,
            settings.libretro_from_ext(&PathBuf::from("")).as_ref()
        );
        assert_eq!(
            None,
            settings.libretro_from_ext(&PathBuf::from(".sfc")).as_ref()
        );
        assert_eq!(
            Some(&PathBuf::from("mesen-s")),
            settings
                .libretro_from_ext(&PathBuf::from("game.sfc"))
                .as_ref()
        );
    }

    #[test]
    fn libretro_from_dir() {
        let mut dir_rules: IndexMap<String, PathBuf> = IndexMap::new();
        dir_rules.insert(
            "/home/user/Emulatoren/games/psx/".to_string(),
            PathBuf::from("mednafen_psx_hw"),
        );
        dir_rules.insert(
            "/home/user/Emulatoren/games/psx*/".to_string(),
            PathBuf::from("swanstation"),
        );

        let settings = super::Settings {
            games: vec![],
            retroarch_arguments: vec![],
            config: None,
            retroarch: Some(PathBuf::from("retroarch")),
            retroarch_config: None,
            libretro: None,
            libretro_directory: None,
            core: None,
            filter: Some(vec!["[!]".to_string()]),
            strict: None,
            which: None,
            which_command: None,
            list_cores: None,
            fullscreen: None,
            resolve: None,
            highlander: Some(true),
            open_config: None,
            config_path: None,
            noconfig: None,
            norun: Some(true),
            nostdin: None,
            cores_rules: None,
            extension_rules: None,
            directory_rules: Some(dir_rules),
        };

        assert_eq!(
            None,
            settings.libretro_from_dir(&PathBuf::from("")).as_ref()
        );
        assert_eq!(
            Some(&PathBuf::from("mednafen_psx_hw")),
            settings
                .libretro_from_dir(&PathBuf::from(
                    "/home/user/Emulatoren/games/psx/Metal Gear Solid.chd"
                ))
                .as_ref()
        );
        assert_eq!(
            Some(&PathBuf::from("swanstation")),
            settings
                .libretro_from_dir(&PathBuf::from(
                    "/home/user/Emulatoren/games/psx⎇/psx_mods/Symphony of the Night (hack).chd"
                ))
                .as_ref()
        );
    }

    #[test]
    fn update_from() -> Result<()> {
        let mut old = super::Settings::new();
        let new = super::Settings {
            games: vec![],
            retroarch_arguments: vec![],
            config: None,
            retroarch: Some(PathBuf::from("retroarch")),
            retroarch_config: None,
            libretro: None,
            libretro_directory: None,
            core: None,
            filter: Some(vec!["[!]".to_string()]),
            strict: None,
            which: None,
            which_command: None,
            list_cores: None,
            fullscreen: None,
            resolve: None,
            highlander: Some(true),
            open_config: None,
            config_path: None,
            noconfig: None,
            norun: Some(true),
            nostdin: None,
            cores_rules: None,
            extension_rules: None,
            directory_rules: None,
        };

        old.update_from(new);
        let updated = old;

        assert_eq!(Some(PathBuf::from("retroarch")), updated.retroarch);
        assert_eq!(Some(vec!["[!]".to_string()]), updated.filter);
        assert_eq!(Vec::<PathBuf>::new(), updated.games);
        assert_eq!(None, updated.noconfig);

        Ok(())
    }

    #[test]
    fn select_game_first() {
        let games: Vec<PathBuf> = ["zelda.smc", "mario.smc", "sonic.md", "game4.gb"]
            .iter()
            .map(PathBuf::from)
            .collect();
        let mut settings = super::Settings {
            games,
            retroarch_arguments: vec![],
            config: None,
            retroarch: Some(PathBuf::from("retroarch")),
            retroarch_config: None,
            libretro: None,
            libretro_directory: None,
            core: None,
            filter: None,
            strict: None,
            which: None,
            which_command: None,
            list_cores: None,
            fullscreen: None,
            resolve: None,
            highlander: None,
            open_config: None,
            config_path: None,
            noconfig: None,
            norun: None,
            nostdin: None,
            cores_rules: None,
            extension_rules: None,
            directory_rules: None,
        };

        assert_eq!(Some(PathBuf::from("zelda.smc")), settings.select_game());

        settings.filter = Some(vec!["m".to_string()]);
        assert_eq!(Some(PathBuf::from("mario.smc")), settings.select_game());

        settings.filter = Some(vec!["gb".to_string()]);
        assert_eq!(None, settings.select_game());
    }
}
