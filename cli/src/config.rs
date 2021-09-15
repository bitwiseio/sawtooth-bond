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

use dirs;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize)]
struct Config {
    rest_api_url: String,
}

fn get_data_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".sawtooth");
    path
}

fn get_config_file() -> PathBuf {
    let mut path = get_data_dir();
    path.push("bond-config.toml");
    path
}

fn create_config_file() -> Result<(), io::Error> {
    if !&get_data_dir().exists() {
        fs::create_dir(&get_data_dir())?;
    }
    fs::File::create(&get_config_file())?;
    Ok(())
}

pub fn set_config(url: &str) -> Result<(), io::Error> {
    let config = Config {
        rest_api_url: url.to_string(),
    };
    let toml = toml::to_string(&config).expect("Unable to set config");

    let config_file = get_config_file();
    if !config_file.exists() {
        create_config_file()?;
    }
    let mut f = fs::File::create(config_file)?;

    f.write_all(toml.as_bytes())?;
    Ok(())
}
