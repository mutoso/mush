/*
    Copyright © 2021 Alastair Feille

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at https://mozilla.org/MPL/2.0/.

    SPDX-License-Identifier: MPL-2.0
*/

use std::{io,
          io::Write,
          process,
          process::Command};

fn main()
{
    loop
    {
        // Print prompt
        print!("> ");
        io::stdout().flush().expect("failed to print prompt");

        // Read in line
        let mut line = String::new();
        io::stdin().read_line(&mut line)
                   .expect("failed to read from stdin");

        // Skip empty lines
        if line.trim().is_empty()
        {
            continue;
        }

        // Split line into command and arguments
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let (head, args) = tokens.split_at(1);
        if let Some(cmd) = head.get(0)
        {
            if cmd.to_string() == "exit"
            {
                process::exit(0);
            }
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
