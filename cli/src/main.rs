// Copyright 2018 Bitwise IO
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate dirs;
extern crate simple_logger;
extern crate toml;
#[macro_use]
extern crate log;

mod commands;
mod config;

use clap::ArgMatches;
use commands::init;
use log::LogLevel;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = parse_args();

    let logger = match args.occurrences_of("verbose") {
        1 => simple_logger::init_with_level(LogLevel::Info),
        2 => simple_logger::init_with_level(LogLevel::Debug),
        0 | _ => simple_logger::init_with_level(LogLevel::Warn),
    };
    logger.expect("Logger was not created");

    let result = match args.subcommand() {
        ("init", Some(args)) => init::run(args),
        _ => unreachable!(),
    };

    std::process::exit(match result {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}

fn parse_args<'a>() -> ArgMatches<'a> {
    let args = clap_app!(bond_cli =>
        (name: "bond-cli")
        (version: VERSION)
        (author: "Bitwise IO")
        (about: "Command-line interface for the Sawtooth Bond application")
        (@setting SubcommandRequiredElseHelp)
        (@arg verbose: -v... "Sets the logging verbosity level")
        (@subcommand init =>
            (about: "Sets the URL of the Bond REST API")
            (@arg url: --url +takes_value +required "URL of the Bond REST API"))
    );
    args.get_matches()
}
