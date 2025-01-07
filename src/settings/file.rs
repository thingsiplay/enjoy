use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use path_absolutize::Absolutize;

/// Opens a file with the associated default application.  It must be af file, not a folder.
pub fn open_with_default(file: &Path) -> Result<(), Box<dyn Error>> {
    let fullpath: PathBuf = match to_fullpath(file, false) {
        Some(fullpath) => fullpath,
        None => return Err("Problem finding the config file.".into()),
    };

    if fullpath.is_file() {
        open::that(fullpath)?;
    } else {
        return Err(format!(
            "Path to config is not accessible or a file: {}",
            fullpath.display()
        )
        .into());
    }

    Ok(())
}

/// Expands tilde and environmental variables in a `Path` and canonicalize to fullpath into a
/// `PathBuf`.  `None` if not possible.
pub fn to_fullpath(file: &Path, canonicalize: bool) -> Option<PathBuf> {
    match shellexpand::full(&file.display().to_string()) {
        Ok(path) => {
            let p = PathBuf::from(path.to_string());
            if canonicalize {
                match p.canonicalize() {
                    Ok(fullpath) => Some(fullpath),
                    Err(_) => None,
                }
            } else {
                match p.absolutize() {
                    Ok(fullpath) => Some(fullpath.into_owned()),
                    Err(_) => None,
                }
            }
        }
        Err(_) => None,
    }
}

/// Expand the tilde in a `Path` and create a `PathBuf` from it.
pub fn tilde(file: &Path) -> PathBuf {
    PathBuf::from(shellexpand::tilde(&file.display().to_string()).into_owned())
}

/// Convert an optional `PathBuf` into a `String`.  `None` is translated into an empty `String`.
pub fn to_str(file: Option<&PathBuf>) -> String {
    match file {
        Some(path) => path.display().to_string(),
        None => String::new(),
    }
}

/// Check if filename (including extension) ends with a specific text and add if its missing.
/// The extension is part of `endswith` suffix check.
pub fn endswith(endswith: &str, mut file: PathBuf) -> PathBuf {
    if !endswith.is_empty() {
        let filename: &str = file
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        if !filename.ends_with(endswith) {
            file = file.with_file_name(format!("{filename}{endswith}"));
        }
    }

    file
}

/// Simply remove last character, if it is a slash.
pub fn trim_last_slash(mut path: String) -> String {
    if path.ends_with('/') {
        path.pop();
    }

    path
}

#[cfg(test)]
mod tests {

    use std::env;
    use std::path::PathBuf;

    // Untested:
    //  - open_with_default()

    #[test]
    fn to_fullpath_empty() {
        let path: PathBuf = PathBuf::from("");
        let output = super::to_fullpath(&path, false);

        assert_eq!(output, None);
    }

    #[test]
    fn to_fullpath_root() {
        let path: PathBuf = PathBuf::from("/");
        let output = super::to_fullpath(&path, false);

        assert_eq!(output, Some(PathBuf::from("/")));
    }

    #[test]
    fn to_fullpath_above_home() {
        let path: PathBuf = PathBuf::from("$HOME/../");
        let output = super::to_fullpath(&path, false);

        assert_eq!(output, Some(PathBuf::from("/home")));
    }

    #[test]
    fn to_fullpath_does_not_exist() {
        let path: PathBuf = PathBuf::from("~/../../bin/filedoesnotexist!(@)/$+");
        let output = super::to_fullpath(&path, false);

        assert_eq!(output, None);
    }

    #[test]
    fn tilde_tilde_only() {
        let path: PathBuf = PathBuf::from("~");
        let output = super::tilde(&path);
        let home = env::var("HOME").unwrap();

        assert_eq!(output, PathBuf::from(home));
    }

    #[test]
    fn tilde_directory() {
        let path: PathBuf = PathBuf::from("~/.config/enjoy");
        let output = super::tilde(&path);
        let home = env::var("HOME").unwrap();

        assert_eq!(output, PathBuf::from(format!("{}/.config/enjoy", home)));
    }

    #[test]
    fn to_str_basic_file() {
        let path: PathBuf = PathBuf::from("/home/user/.vimrc");
        let output = super::to_str(Some(&path));

        assert_eq!(output, "/home/user/.vimrc".to_string());
    }

    #[test]
    fn endswith_libretroso_no_need() {
        let path: PathBuf = PathBuf::from("snes9x_libretro.so");
        let output = super::endswith("_libretro.so", path);

        assert_eq!(output, PathBuf::from("snes9x_libretro.so"));
    }

    #[test]
    fn endswith_libretroso_add_suffix() {
        let path: PathBuf = PathBuf::from("snes9x");
        let output = super::endswith("_libretro.so", path);

        assert_eq!(output, PathBuf::from("snes9x_libretro.so"));
    }

    #[test]
    fn endswith_libretroso_missing_ext() {
        let path: PathBuf = PathBuf::from("snes9x_libretro");
        let output = super::endswith("_libretro.so", path);

        assert_eq!(output, PathBuf::from("snes9x_libretro_libretro.so"));
    }
}
