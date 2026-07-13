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

use std::collections::HashSet;
use std::fmt::Write;

use anyhow::Context;
use anyhow::Result;
use crate::hii::extract;
use crate::hii::forms;
use crate::hii::forms::list_questions;
use crate::hii::package;
use crate::types::Backend;
use crate::types::GetResponse;
use crate::types::GetResponseList;
use crate::types::HiiDatabase;
use crate::types::HiiShowIfrResponse;
use crate::types::HiiStringsPackage;
use crate::types::Question;
use crate::types::SetResponse;
use crate::types::SetResponseList;

/// `SettingsBackend` is a trait which should be satisfied by all backends (currently only hii)
pub trait SettingsBackend {
    /// set changes the value of the UEFI question/attribute
    ///
    /// # Errors
    /// Returns an error if the backend fails to apply the setting.
    fn set(question: &str, new_value: &str, selector: Option<&str>) -> Result<SetResponseList>;
    /// get displays the value of the UEFI question/attribute
    ///
    /// # Errors
    /// Returns an error if the backend fails to read the setting.
    fn get(question: &str, selector: Option<&str>) -> Result<GetResponseList>;
}

pub struct HiiBackend {}

impl HiiBackend {
    /// `extract_db` extracts `HiiDB` from efivarfs and returns it in bytes (in `HiiDatabase`'s db field which is Vec<u8>)
    ///
    /// # Errors
    /// Returns an error if the `HiiDB` cannot be read from efivarfs.
    pub fn extract_db() -> Result<HiiDatabase> {
        let resp = HiiDatabase {
            db: extract::extract_db()?,
        };
        Ok(resp)
    }

    /// `parse_db` reads the `HiiDB` from bytes and returns a `ParsedHiiDB` struct.
    ///
    /// # Errors
    /// Returns an error if the `HiiDB` bytes cannot be parsed.
    pub fn parse_db() -> Result<package::ParsedHiiDB> {
        let db_bytes = extract::extract_db()?;
        package::read_db(&db_bytes)
    }

    /// `get_one` gets the value of a single question from the `ParsedHiiDB` struct.
    /// 
    /// # Errors
    /// Returns an error if database is malformed.
    pub fn get_one(question: &str, parsed_db: &package::ParsedHiiDB) -> Result<GetResponseList> {
        let question_variations = HashSet::from([question]);

        let mut resp = Vec::new();

        for (guid, package_list) in &parsed_db.forms {
            for form_package in package_list {
                if let Some(question_descriptor) = forms::find_question(
                    form_package.clone(),
                    parsed_db
                        .strings
                        .get(guid)
                        .context(format!("Failed to get string packages using GUID {guid}"))?,
                    &question_variations,
                ) {
                    let mut get_resp = GetResponse {
                        selector: guid.clone(),
                        backend: Backend::Hii,
                        question: Question {
                            name: question_descriptor.question,
                            answer: question_descriptor.value,
                            help: question_descriptor.help,
                            ..Default::default()
                        },
                    };

                    for opt in question_descriptor.possible_options {
                        get_resp.question.options.push(opt.value);
                    }

                    resp.push(get_resp);
                }
            }
        }

        Ok(GetResponseList {
            responses: resp,
        })
    }

    /// `show_ifr` returns a human readable representation of the forms in Hii
    ///
    /// # Errors
    /// Returns an error if the `HiiDB` bytes cannot be parsed.
    pub fn show_ifr(db_bytes: &[u8]) -> Result<HiiShowIfrResponse> {
        // We depend on the caller to provide us with the hiidb instead of calling extract here
        // because they might want to provide a file instead.

        let mut readable_representation = String::new();

        let parsed_db = package::read_db(db_bytes)?;

        for (guid, package_list) in parsed_db.forms {
            write!(readable_representation, "Packagelist {}", &guid)?;
            for form_package in package_list {
                readable_representation.push_str(&forms::display(
                    form_package,
                    0,
                    parsed_db
                        .strings
                        .get(&guid)
                        .context(format!("Failed to get string packages using GUID {guid}"))?,
                )?);
            }
        }

        let resp = HiiShowIfrResponse {
            readable_representation,
        };
        Ok(resp)
    }

    /// list all strings-id, string pairs in `HiiDB`
    ///
    /// # Errors
    /// Returns an error if the `HiiDB` bytes cannot be parsed.
    pub fn list_strings(db_bytes: &[u8]) -> Result<Vec<HiiStringsPackage>> {
        let mut resp = Vec::new();
        let parsed_db = package::read_db(db_bytes)?;

        for (guid, package_list) in parsed_db.strings {
            for string_package in package_list {
                let mut p = HiiStringsPackage {
                    package_list: guid.clone(),
                    language: string_package.0,
                    ..Default::default()
                };
                for (string_id, string) in string_package.1 {
                    p.string_package.insert(string_id, string);
                }
                resp.push(p);
            }
        }

        Ok(resp)
    }

    /// # Errors
    /// Returns an error if the `HiiDB` bytes cannot be parsed.
    pub fn list_questions(db_bytes: &[u8]) -> Result<Vec<Question>> {
        let mut res = Vec::new();
        let parsed_db = package::read_db(db_bytes)?;
        for (guid, package_list) in parsed_db.forms {
            let string_packages = parsed_db
                .strings
                .get(&guid)
                .context(format!("Failed to get string packages using GUID {guid}"))?;

            for form_package in package_list {
                //  TODO: show form name, package list guid in the final response as well

                for question_descriptor in list_questions(form_package, string_packages) {
                    let mut question = Question {
                        name: question_descriptor.question,
                        answer: question_descriptor.value,
                        help: question_descriptor.help,
                        ..Default::default()
                    };
                    for opt in question_descriptor.possible_options {
                        question.options.push(opt.value);
                    }
                    // don't show it if everything is empty
                    if !(question.name.is_empty()
                        && question.answer.is_empty()
                        && question.help.is_empty()
                        && question.options.is_empty())
                    {
                        res.push(question);
                    }
                }
            }
        }
        Ok(res)
    }
}

impl SettingsBackend for HiiBackend {
    fn set(question: &str, new_value: &str, _selector: Option<&str>) -> Result<SetResponseList> {
        // TODO: use selector to only change values which match question + selector

        let mut resp = Vec::new();

        let db_bytes = extract::extract_db()?;
        let parsed_db = package::read_db(&db_bytes)?;

        let question_variations = HashSet::from([question]);

        for (guid, package_list) in parsed_db.forms {
            let string_packages = parsed_db
                .strings
                .get(&guid)
                .context(format!("Failed to get string packages using GUID {guid}"))?;

            for form_package in package_list {
                // try to find the question
                if let Some(question_descriptor) =
                    forms::find_question(form_package, string_packages, &question_variations)
                {
                    let mut modified = false;
                    // if the question_descriptor provides options then set the closest one from new_value_variations
                    // (example whatever matches from [Enabled, Enable])
                    // else try setting the new_value because it might be some arbitrary value like a number
                    // (will return error if doesn't match constraints)
                    if question_descriptor.possible_options.is_empty() {
                        modified = forms::change_value(&question_descriptor, new_value)?;
                    } else {
                        // This is different from modified because if varstore doesn't exist for the question
                        // then we can't set answers but it isn't an error.
                        let mut found_option = false;
                        for opt in &(question_descriptor.possible_options) {
                            if new_value.eq_ignore_ascii_case(&(opt.value)) {
                                found_option = true;
                                modified = forms::change_value(&question_descriptor, &(opt.value))?;
                                break;
                            }
                        }
                        // if not a single option matched then error out
                        // we went though all options and if it still wasn't modified then this isn't in the options
                        if !found_option {
                            return Err(forms::ChangeValueError::InvalidOption.into());
                        }
                    }

                    if modified {
                        let mut set_resp = SetResponse {
                            selector: guid.clone(),
                            backend: Backend::Hii,
                            question: Question {
                                name: question_descriptor.question,
                                answer: new_value.to_owned(),
                                help: question_descriptor.help,
                                ..Default::default()
                            },
                            modified: true,
                        };

                        for opt in question_descriptor.possible_options {
                            set_resp.question.options.push(opt.value);
                        }

                        resp.push(set_resp);
                    }
                }
                // question not found in this form package
            }
        }

        Ok(SetResponseList {
            responses: resp,
        })
    }

    fn get(question: &str, _selector: Option<&str>) -> Result<GetResponseList> {
        // TODO: use selector to only get questions which match question + selector
        let db_bytes = extract::extract_db()?;
        let parsed_db = package::read_db(&db_bytes)?;
        Self::get_one(question, &parsed_db)
    }
}
