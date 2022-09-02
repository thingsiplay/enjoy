# Changelog

Keep track of changes with every release of https://github.com/thingsiplay/enjoy .

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
