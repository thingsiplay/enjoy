mod settings;

use crate::settings::RunCommand;
use crate::settings::Settings;

use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // The flow of the program is build around the idea of creating a main settings structure from
    // various places.  Some of them require to be created in specific order, as they depend on
    // previously generated settings.  In the next step the main application settings are created
    // and updated by reading the previously created settings.  From this point on the program can
    // extract and build the final command to execute by reading this global universal setting
    // structure.  The last step would be to actually execute the command and finish up the final
    // work.

    let argument_options = Settings::new_from_cmdline(None);

    // Exit program after printing fullpath or opening the user settings ini file.
    if argument_options.print_config() || argument_options.open_config()? {
        return Ok(());
    }

    let user_config = Settings::new_from_config(argument_options.get_config())?;
    let ignore_stdin: bool = argument_options.is_nostdin() || user_config.is_nostdin();
    let stdin_games = Settings::new_from_stdin(ignore_stdin)?;

    let mut app_settings = Settings::new();
    // Overwrite fields in app_settings only, if new fields are Some().
    app_settings.update_from(user_config);
    app_settings.update_from(stdin_games);
    app_settings.update_from(argument_options);

    let mut defaults = Settings::new_from_defaults();
    if !app_settings.is_libretro_path_available() {
        // Extract keys and values from `retroarch.cfg` only if the path to `libretro` installation
        // directory in `RetroArch` is unknown.
        let raconfig = Settings::new_from_retroarch_config(app_settings.get_retroarch_config())?;
        defaults.update_from(raconfig);
    }
    // Overwrite only those keys in `app_settings`, which their values are currently `None`.
    app_settings.update_defaults_from(defaults);

    if app_settings.is_game_available() || app_settings.is_norun() {
        let mut run: RunCommand = app_settings.build_command()?;

        if !app_settings.is_norun() {
            if app_settings.there_can_only_be_one() {
                eprintln!("retroarch process already running. There Can Be Only One!");
            } else {
                run.output = app_settings.run(&mut run.cmdline);
            }
        }
        if app_settings.is_list_cores() {
            for core in app_settings.find_core_match(&run.libretro) {
                println!("{core}");
            }
        }
        if app_settings.is_which_command() {
            print_cmdline(&run.cmdline);
        } else {
            app_settings.print_which(&run.game);
        }
    } else if app_settings.is_list_cores() {
        app_settings.print_cores();
    } else {
        return Err("A path to game is required.".into());
    }

    Ok(())
}

// Prints program name and each commandline arguments exactly the same as it is used to run
// RetroArch.
fn print_cmdline(command: &Command) {
    print!("{:?}", command.get_program());
    for arg in command.get_args() {
        print!(" {arg:?}");
    }
    println!();
}
