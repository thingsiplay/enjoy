use crate::settings::file;

use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use configparser::ini;
use indexmap::map::IndexMap;

/// Check if a process is running.  If `print_pid` is `true`, then print the pid of found process
/// to stdout.
pub fn is_running(process_name: &str, print_pid: bool) -> bool {
    let mut cmdline: Command = Command::new(String::from("pidof"));

    // return one PID only
    cmdline.arg("--single-shot");
    if !print_pid {
        // quiet mode, only set the exit code
        cmdline.arg("-q");
    }

    cmdline
        .arg(process_name)
        .status()
        .expect("Could not execute `pidof` command.")
        .success()
}

/// Searches the default locations for the file `retroarch.cfg`, which is the main
/// configuration file of `RetroArch`.  Their tilde or environment variables are expanded
/// accordingly.  The locations are:
///     1. `$XDG_CONFIG_HOME/retroarch/retroarch.cfg`
///     2. `~/.config/retroarch/retroarch.cfg`
///     3. `~/.retroarch.cfg`
/// ... in that order.
pub fn search_default_config() -> Option<PathBuf> {
    let mut fullpath: PathBuf;

    if let Ok(path) = shellexpand::env("$XDG_CONFIG_HOME/retroarch/retroarch.cfg") {
        fullpath = PathBuf::from(path.to_string());
        if fullpath.exists() {
            return Some(fullpath);
        }
    }

    fullpath = PathBuf::from(shellexpand::tilde("~/.config/retroarch/retroarch.cfg").to_string());
    if fullpath.exists() {
        return Some(fullpath);
    }

    fullpath = PathBuf::from(shellexpand::tilde("~/.retroarch.cfg").to_string());
    if fullpath.exists() {
        return Some(fullpath);
    }

    None
}

/// Parses a `RetroArch` configuration file and returns a `IndexMap` from it.  The format is like
/// a regular INI file without sections.  The set `lookup_keys` contains all key names to look
/// for in the file and extract only those key and value pairs as strings.  The surrounding
/// double quotes are removed from the value.
pub fn parse_retroarch_config(
    path: &Option<PathBuf>,
    lookup_keys: &HashSet<String>,
) -> Result<IndexMap<String, String>, Box<dyn Error>> {
    let mut ini = ini::Ini::new_cs();

    match ini.load(
        &path
            .as_ref()
            .expect("No configuration file.")
            .display()
            .to_string(),
    ) {
        Ok(ini) => Ok(extract_default_inikeys(&ini, lookup_keys)),
        Err(e) => Err(e.into()),
    }
}

// Searches all `lookup_keys` in `default` section of an INI structure and returns a regular
// IndexMap of it.  Empty strings or missing keys are excluded.
fn extract_default_inikeys(
    ini: &IndexMap<String, IndexMap<String, Option<String>>>,
    lookup_keys: &HashSet<String>,
) -> IndexMap<String, String> {
    let mut found_keys: IndexMap<String, String> = IndexMap::new();

    for (key, value) in ini
        .get("default")
        .unwrap()
        .iter()
        .filter(|(k, _)| lookup_keys.contains(k.as_str()))
        .map(|(k, v)| (k.to_string(), v.as_ref().unwrap()))
    {
        found_keys.insert(key, value.trim_matches('"').to_string());
    }

    found_keys
}

/// Combine the `libretro-directory` and `libretro` core file to a fullpath.  Add a string to
/// the end of the filename, if it does not end like that.  This includes the file extension
/// and end of the filename part.  In example the common "_libretro.so" could be added.
pub fn libretro_fullpath(
    directory: Option<PathBuf>,
    libretro: Option<PathBuf>,
    endswith: &str,
) -> Option<PathBuf> {
    let mut fullpath: PathBuf = PathBuf::new();

    if let Some(dir) = directory {
        fullpath = file::tilde(&dir);
    };
    fullpath = fullpath.join(file::tilde(&libretro.unwrap_or_default()));
    fullpath = file::endswith(endswith, fullpath);

    file::to_fullpath(&fullpath, false)
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use configparser::ini;
    use indexmap::map::IndexMap;

    // Untested:
    //  - search_default_config()
    //  - libretro_fullpath()

    #[test]
    fn is_running_cargo() {
        assert!(super::is_running("cargo", false));
    }

    #[test]
    fn is_running_empty() {
        assert!(!super::is_running("", false));
    }

    #[test]
    fn extract_default_inikeys_single() {
        let inidata: IndexMap<String, IndexMap<String, Option<String>>> = ini::Ini::new()
            .read(String::from(
                "
                video_vsync = \"true\"
                libretro_directory = \"/home/user/.config/retroarch/cores\"
                audio_device = \"\"
                ",
            ))
            .expect("Could not create inidata.");

        let mut lookup_keys: HashSet<String> = HashSet::new();
        lookup_keys.insert("libretro_directory".to_string());

        let found_keys = super::extract_default_inikeys(&inidata, &lookup_keys);

        assert_eq!(
            "/home/user/.config/retroarch/cores".to_string(),
            found_keys.get("libretro_directory").unwrap().to_string()
        );
        assert_eq!(None, found_keys.get("video_vsync"));
    }

    #[test]
    fn extract_default_inikeys_multiple() {
        let inidata: IndexMap<String, IndexMap<String, Option<String>>> = ini::Ini::new()
            .read(String::from(
                "
                video_vsync = \"true\"
                libretro_directory = \"Ramírez\"
                libretro_directory = \"/home/user/.config/retroarch/cores\"
                audio_device = \"\"
                ",
            ))
            .expect("Could not create inidata.");

        let mut lookup_keys: HashSet<String> = HashSet::new();
        lookup_keys.insert("audio_device".to_string());
        lookup_keys.insert("video_vsync".to_string());
        lookup_keys.insert("libretro_directory".to_string());

        let found_keys = super::extract_default_inikeys(&inidata, &lookup_keys);

        assert_eq!(
            "".to_string(),
            found_keys.get("audio_device").unwrap().to_string()
        );
        assert_eq!(
            "true".to_string(),
            found_keys.get("video_vsync").unwrap().to_string()
        );
        assert_eq!(
            "/home/user/.config/retroarch/cores".to_string(),
            found_keys.get("libretro_directory").unwrap().to_string()
        );
    }
}
