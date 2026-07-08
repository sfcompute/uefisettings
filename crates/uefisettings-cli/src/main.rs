// Copyright 2023 Meta Platforms, Inc. and affiliates.
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use log::info;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use uefisettings::exports::HiiBackend;
use uefisettings::exports::SettingsBackend;
use uefisettings::types::GetResponseList;
use uefisettings::types::HiiStringsPackage;
use uefisettings::types::Question;
use uefisettings::types::SetResponseList;

const MAX_ALLOWED_FILESIZE: u64 = 16 * 1024 * 1024;

#[derive(Debug, Parser)]
#[command(name = "uefisettings", about = "UEFI Settings Manipulation Tool", long_about = None)]
struct UefiSettingsToolArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Commands which work on machines exposing the UEFI `HiiDB`
    Hii(HiiCommand),
    /// Auto-identify backend and get the current value of a question
    Get {
        question: String,
        #[arg(short = 'j', long = "json")]
        json: bool,
    },
    /// Auto-identify backend and set/change the value of a question
    Set {
        question: String,
        value: String,
        #[arg(short = 'j', long = "json")]
        json: bool,
    },
}

#[derive(Debug, Parser)]
struct HiiCommand {
    #[command(subcommand)]
    command: HiiSubcommands,
}

#[derive(Debug, Subcommand)]
enum HiiSubcommands {
    /// Get the current value of a question
    Get {
        question: String,
        #[arg(short = 'j', long = "json")]
        json: bool,
    },
    /// Set/change the value of a question
    Set {
        question: String,
        value: String,
        #[arg(short = 'j', long = "json")]
        json: bool,
    },
    /// Show a human readable representation of the Hii Forms
    ShowIfr {
        /// If filename of `HiiDB` isn't specified then this tool will try to automatically extract it
        #[arg(short, long)]
        filename: Option<PathBuf>,
    },
    /// Dump `HiiDB` into a file
    ExtractDb { filename: PathBuf },
    /// List all strings-id, string pairs in `HiiDB`
    ListStrings {
        /// If filename of `HiiDB` isn't specified then this tool will try to automatically extract it
        #[arg(short, long)]
        filename: Option<PathBuf>,
        #[arg(short = 'j', long = "json")]
        json: bool,
    },
}

fn main() {
    env_logger::init();
    let args = UefiSettingsToolArgs::parse();

    if let Err(why) = handle_cmds(&args) {
        println!("{}", json!({ "error_message": format!("{why:#}") }));
        process::exit(1);
    }

    info!("Exiting UEFI Settings Manipulation Tool");
}

fn handle_cmds(args: &UefiSettingsToolArgs) -> Result<()> {
    match &args.command {
        Commands::Hii(hii_command) => match &hii_command.command {
            HiiSubcommands::Get { question, json } => {
                let res = HiiBackend::get(question, None)?;
                print_get(&res, *json);
            }
            HiiSubcommands::Set {
                question,
                value,
                json,
            } => {
                let res = HiiBackend::set(question, value, None)?;
                print_set(&res, *json);
            }
            HiiSubcommands::ShowIfr { filename } => {
                let res = HiiBackend::show_ifr(&get_db_dump_bytes(filename.as_deref())?)?;
                println!("{}", res.readable_representation);
            }
            HiiSubcommands::ExtractDb { filename } => {
                let mut file = File::create(filename)?;
                let res = HiiBackend::extract_db()?;
                file.write_all(&res.db)?;
                println!("{}", json!({ "info": format!("HiiDB written to {}", filename.display()) }));
            }
            HiiSubcommands::ListStrings { filename, json } => {
                let res = HiiBackend::list_strings(&get_db_dump_bytes(filename.as_deref())?)?;
                print_strings(&res, *json);
            }
        },
        // Only the Hii backend is supported, so "auto-identify" resolves to Hii.
        Commands::Get { question, json } => {
            let res = HiiBackend::get(question, None)?;
            print_get(&res, *json);
        }
        Commands::Set {
            question,
            value,
            json,
        } => {
            let res = HiiBackend::set(question, value, None)?;
            print_set(&res, *json);
        }
    }
    Ok(())
}

fn get_db_dump_bytes(filename: Option<&Path>) -> Result<Vec<u8>> {
    // If dbdump's path is provided use that
    if let Some(dbdump_path) = filename {
        info!("Using database dump from file: {}", dbdump_path.display());

        let mut file = File::open(dbdump_path)
            .context(format!("opening dbdump from {}", dbdump_path.display()))?;

        // Most Hii DBs are a few hundred kilobytes in size and the largest we've seen so far is close to 3 MB.
        // Since we're reading the entire DB into a Vec<u8> we need to have a check here.
        if file
            .metadata()
            .context("failed to read metadata for open file")?
            .len()
            > MAX_ALLOWED_FILESIZE
        {
            return Err(anyhow!(
                "File size is too big for the file to be a HII database."
            ));
        }

        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)
            .context("Couldn't convert file bytes to Vec<u8>")?;

        Ok(file_contents)
    } else {
        Ok(HiiBackend::extract_db()?.db)
    }
}

fn question_to_json(question: &Question) -> Value {
    json!({
        "name": question.name,
        "answer": question.answer,
        "options": question.options,
        "help": question.help,
    })
}

fn print_get(list: &GetResponseList, json: bool) {
    let value = json!({
        "responses": list
            .responses
            .iter()
            .map(|r| json!({
                "selector": r.selector,
                "backend": format!("{:?}", r.backend),
                "is_translated": r.is_translated,
                "question": question_to_json(&r.question),
            }))
            .collect::<Vec<Value>>(),
    });
    print_value(&value, json);
}

fn print_set(list: &SetResponseList, json: bool) {
    let value = json!({
        "responses": list
            .responses
            .iter()
            .map(|r| json!({
                "selector": r.selector,
                "backend": format!("{:?}", r.backend),
                "is_translated": r.is_translated,
                "modified": r.modified,
                "question": question_to_json(&r.question),
            }))
            .collect::<Vec<Value>>(),
    });
    print_value(&value, json);
}

fn print_strings(packages: &[HiiStringsPackage], json: bool) {
    let value = Value::Array(
        packages
            .iter()
            .map(|p| {
                let mut strings = Map::new();
                for (id, string) in &p.string_package {
                    strings.insert(id.to_string(), Value::String(string.clone()));
                }
                json!({
                    "package_list": p.package_list,
                    "language": p.language,
                    "string_package": Value::Object(strings),
                })
            })
            .collect::<Vec<Value>>(),
    );
    print_value(&value, json);
}

// print_value either prints compact json or a pretty-printed representation.
fn print_value(value: &Value, json: bool) {
    if json {
        println!("{value}");
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
        );
    }
}
