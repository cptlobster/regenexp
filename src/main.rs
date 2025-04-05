// <one line to give the program's name and a brief idea of what it does.>
// Copyright (C) 2025 Dustin Thomas <stdio@cptlobster.dev>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//
use clap::Parser;
use std::io;
use log::debug;

/// REgular GENerative EXpressions: Generate random strings based on regex criteria
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input regex. If not defined here, it will be read from stdin
    exp: Option<String>
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let exp = match args.exp {
        Some(exp) => {
            debug!("Reading expression from argument...");
            exp
        },
        None => {
            debug!("Reading expression from stdin...");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            buffer
        }
    };

    println!("{}", exp);
    Ok(())
}
