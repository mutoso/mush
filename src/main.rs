/*
    Copyright © 2021 Alastair Feille

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at https://mozilla.org/MPL/2.0/.

    SPDX-License-Identifier: MPL-2.0
*/

use std::{env,
          fs,
          io,
          io::Write,
          path::PathBuf,
          process,
          process::Command};

use chrono::Local;

mod history;

fn main()
{
    let session_time = Local::now();
    let mut fallback_mode = false;
    loop
    {
        let cwd: String = env::current_dir().map(|p| {
                                                // Attempt to canonicalize the path else return "???"
                                                fs::canonicalize(p).unwrap_or("???".into())
                                                                   .to_string_lossy()
                                                                   .into()
                                            })
                                            // Attempt to get the current path else return "???"
                                            .unwrap_or("???".into());
        // Print prompt
        print!("{}{}@{}:{}> ",
               if fallback_mode { "(bash) " } else { "" },
               whoami::username(),
               whoami::hostname(),
               cwd);
        io::stdout().flush().expect("failed to print prompt");

        // Read in line
        let mut line = String::new();
        io::stdin().read_line(&mut line)
                   .expect("failed to read from stdin");

        let line = line.trim();
        // Skip empty lines
        if line.is_empty()
        {
            continue;
        }
        if line == "\u{0014}"
        {
            // toggle fallback mode
            fallback_mode = !fallback_mode;
            continue;
        }

        // Split line into command and arguments
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let (head, args) = tokens.split_at(1);
        if let Some(cmd) = head.get(0)
        {
            history::log(session_time,
                         Local::now(),
                         line.to_string(),
                         env::current_dir().ok(),
                         fallback_mode);
            if cmd.to_string() == "exit"
            {
                process::exit(0);
            }
            if cmd.to_string() == "cd"
            {
                let path_string = args.join(" ");
                // if no directory is given
                let p = if path_string.trim().is_empty()
                {
                    // change into the home directory
                    dirs::home_dir().expect("can't get home directory")
                }
                else
                {
                    // use the given directory
                    PathBuf::from(path_string)
                };

                if let Err(e) = env::set_current_dir(&p)
                {
                    eprintln!("mush: cd: {}: {}", p.display(), e);
                }
                continue;
            }
            let (cmd, args) = if fallback_mode
            {
                (&"bash", vec!["-c", line])
            }
            else
            {
                (cmd, Vec::from(args))
            };
            match Command::new(cmd).args(args).spawn()
            {
                Ok(mut child) =>
                {
                    let _ecode = child.wait().expect("failed to wait on child");
                },
                Err(e) => eprintln!("mush: could not run command {}: {}", cmd, e),
            }
        }
    }
}
