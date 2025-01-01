// core/args.rs
use clap::Parser;

#[derive(Debug, Parser)]
pub struct HelloArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: String,
}

#[derive(Debug, Parser)]
pub struct ByeArgs {
    /// Name of the person to bid farewell
    #[arg(short, long)]
    pub name: String,
}

