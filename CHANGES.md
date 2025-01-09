# Changelog

Keep track of changes for [enjoy](https://github.com/thingsiplay/enjoy) .

## [0.6.0] - January 10, 2025

- changed: symbolic links are no longer followed for all files, meaning that
  paths are passed over to RetroArch executable as symlink if its one, instead
  resolving to its target
- new: option `-l`, `--resolve` to follow symbolic links on Rom files
- new: option `-s`, `--shader` to force a specific shader preset
- new: option `-S`, `--shader-directory` to set a specific directory for
  `--shader`
- changed: previous short option `-s` for `--strict` changed to `-p`
- removed: no debug builds provided anymore

## [0.5.0] - April 18, 2024

- changed: the options parser and help text from `-h` or `--help` has no
  longer colored output and is reorganized
- changed: if an empty `""` entry is included, then entire program assumes
  nothing is given, in example following does not work: `enjoy mario.smc ""`,
- changed: logo slightly updated and corrected
- changed: pandoc no longer used to convert README.md to HTML with Makefile
- renamed: CHANGELOG.md to CHANGES.md
- removed: install.sh and uninstall.sh scripts removed
- internal: some code refactor, formatting and dependency upgrades

## [0.4.0] - September 18, 2022

- new: option `-v`, `--version` to display the current version information of
  the app
- new: option `-s`, `--strict` to turn option `--filter` into strict mode, case
  sensitive and no longer adding stars around the pattern automatically
  (meaning it matches exactly the name and not somewhere in the middle of
  filename), example: `-sf "Super"` won't match "Super Mario Land.gb", but
  `-sf "Super*"` will
- changed: `-f`, `--filter` can be used multiple times, all of them have to
  match to get a result (works only as commandline option)
- changed: multiple custom core names under section `[cores]` in user settings
  ini file can be specified in one line separated by space, example:
  `gb gbc = sameboy`
- changed: if a game ROM is not found and option `-x` is active, then
  simulation of the process will continue and no longer stops, this allows
  running other options such as `-n` with non existing games to quickly check
  associated file extension, example: `-xn ..sfc`
- changed: wildcard support for directory rules implemented, only supported
  wildcards are star `*` (none or any number of characters) and question mark
  `?` (exactly one any character), example: `[~/Emulatoren/games/psx\*]`
- changed: directory rules are exact match now, no longer is it compared if the
  game path "starts" with the directory path, also trailing basckslash is
  optional
- changed: internal library updated to respect the order of rules in the
  configuration for priority reasons, first match of a rule will be used, this
  is not considered a bug, because the order was not important before
- new: option `-o`, `--config-path` to print the fullpath of the user settings
  ini file
- changed: previous short option `-o` for `--open-config` is renamed to`-O` and
  will no longer print path
- changed: logo rework

## [0.3.0] - September 2, 2022

- new: option `-n` and `--list-cores` to list all custom core names in section
  "[cores]" from user configuration, if a game is given too then only matching
  cores to the game will be printed
- new: option `-W` and `--which-command`, similar to `--which` but will print
  complete commandline used to run RetroArch
- changed: option `-o` and `--open-config` will now print path to the config too
- changed: better error message if a game file is not found, pointing to the
  file it was looking for
- bug: annoying error message when game run just fine, now checks for
  "exit status: 0" instead "exit code: 0"
- internal: replaced or updated some backend libraries
- internal: updated code base to Rust Edition to 2021

## [0.2.0] - June 8, 2021

- added new option `--` to bypass arguments directly to `retroarch`
- new logo design

## [0.1.1] - June 07, 2021

- documentation rework and new Wiki created
- little internal code refactoring

## [0.1.0] - May 31, 2021

- initial upload
