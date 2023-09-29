// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use args::Arguments;
use clap::Parser;

use std::{process::exit};

mod args;

const EXIT_SUCCESS: i32 = 0;
// const EXIT_INVALID_PARAMETER: i32 = 1;

fn main() {
    let args = Arguments::parse();
    println!("args: {:?}", args);
    exit(EXIT_SUCCESS);
}
