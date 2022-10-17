use clap::{Parser, Subcommand, ValueEnum};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// please input a Service cmd Operate
    Service {
        #[arg(value_enum, required = true, value_name = "CMD")]
        cmd: ServiceCmd,
        #[arg(
            required = false,
            value_name = "MODE",
            default_value = None,
            default_value_if("cmd",  "start",  Some("d"))
        )]
        mode: Option<String>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ServiceCmd {
    Start,
    Stop,
}
