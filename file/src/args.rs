// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "file", version = "0.0.1", about = "Manage file system objects", long_about = None)]
pub struct Arguments {

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub enum ConfigSubCommand {
    #[clap(name = "get", about = "Get the current file object configuration.")]
    Get,
    #[clap(name = "set", about = "Set the file object configuration.")]
    Set,
    #[clap(name = "test", about = "Check if the file object matches the desired configuration, or does not match.")]
    Test,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub enum SubCommand {
    #[clap(name = "config", about = "Manage file system object configuration.", arg_required_else_help = true)]
    Config {
        #[clap(subcommand)]
        subcommand: ConfigSubCommand,
    }
}
