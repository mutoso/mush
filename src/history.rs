/*
    Copyright © 2021 Alastair Feille

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at https://mozilla.org/MPL/2.0/.

    SPDX-License-Identifier: MPL-2.0
*/

use std::{fs::OpenOptions,
          path::PathBuf};

use chrono::{DateTime,
             FixedOffset,
             Local,
             SubsecRound};

use serde::Serialize;

#[derive(Debug, Serialize)]
struct Entry
{
    #[serde(rename = "Session started")]
    session_time:  DateTime<FixedOffset>,
    #[serde(rename = "Command started")]
    command_time:  DateTime<FixedOffset>,
    #[serde(rename = "Command")]
    line:          String,
    #[serde(rename = "Current directory")]
    cwd:           Option<PathBuf>,
    #[serde(rename = "Fallback mode")]
    fallback_mode: bool,
}

pub fn log(session_time: DateTime<Local>,
           command_time: DateTime<Local>,
           line: String,
           cwd: Option<PathBuf>,
           fallback_mode: bool)
{
    // Truncate the times to microseconds
    let session_time = session_time.trunc_subsecs(6);
    let command_time = command_time.trunc_subsecs(6);

    let entry = Entry { session_time: session_time.with_timezone(session_time.offset()),
                        command_time: command_time.with_timezone(command_time.offset()),
                        line,
                        cwd,
                        fallback_mode };

    let home_path = dirs::home_dir();
    if home_path.is_none()
    {
        eprintln!("mush: can't access home");
        return;
    }

    let mut history_path = PathBuf::from(home_path.unwrap());
    history_path.push(".mush_history.csv");

    let history_already_exists = history_path.exists() && history_path.is_file();

    let file = OpenOptions::new().create(true)
                                 .append(true)
                                 .open(&history_path);
    if let Err(e) = file
    {
        eprintln!("mush: can't open history: {}", e);
        return;
    }

    let file = file.unwrap();

    let mut wtr = csv::WriterBuilder::new().has_headers(!history_already_exists)
                                           .from_writer(file);

    if let Err(e) = wtr.serialize(entry)
    {
        eprintln!("mush: can't log history: {}", e);
    }

    if let Err(e) = wtr.flush()
    {
        eprintln!("mush: can't log history: {}", e);
    }
}
