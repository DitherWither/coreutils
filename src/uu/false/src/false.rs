//  * This file is part of the uutils coreutils package.
//  *
//  * For the full copyright and license information, please view the LICENSE
//  * file that was distributed with this source code.
use clap::{Arg, ArgAction, Command};
use std::{ffi::OsString, io::Write};
use uucore::error::{set_exit_code, UResult};
use uucore::help_about;

const ABOUT: &str = help_about!("false.md");

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let mut command = uu_app();

    // Mirror GNU options, always return `1`. In particular even the 'successful' cases of no-op,
    // and the interrupted display of help and version should return `1`. Also, we return Ok in all
    // paths to avoid the allocation of an error object, an operation that could, in theory, fail
    // and unwind through the standard library allocation handling machinery.
    set_exit_code(1);

    let args: Vec<OsString> = args.collect();
    if args.len() > 2 {
        return Ok(());
    }

    if let Err(e) = command.try_get_matches_from_mut(args) {
        let error = match e.kind() {
            clap::error::ErrorKind::DisplayHelp => command.print_help(),
            clap::error::ErrorKind::DisplayVersion => {
                writeln!(std::io::stdout(), "{}", command.render_version())
            }
            _ => Ok(()),
        };

        // Try to display this error.
        if let Err(print_fail) = error {
            // Completely ignore any error here, no more failover and we will fail in any case.
            let _ = writeln!(std::io::stderr(), "{}: {}", uucore::util_name(), print_fail);
        }
    }

    Ok(())
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(clap::crate_version!())
        .about(ABOUT)
        // We provide our own help and version options, to ensure maximum compatibility with GNU.
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(
            Arg::new("help")
                .long("help")
                .help("Print help information")
                .action(ArgAction::Help),
        )
        .arg(
            Arg::new("version")
                .long("version")
                .help("Print version information")
                .action(ArgAction::Version),
        )
}
