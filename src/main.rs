mod core {
    pub mod target {
        pub mod host;
        pub mod range;
        pub mod subnet;
    }
	pub mod auth;
}
mod modules {
	pub mod check;
    pub mod model;
} 

use clap::{ArgGroup, Parser, Subcommand};
use core::target::{host::t_host, subnet::t_subnet, range::t_range};
use core::auth::Auth;
use modules::model::model;
use modules::check::version;

#[derive(Debug, Parser)]
#[command(
    name = "Axis-Toolkit",
    version = "1.0",
    author = "Your Name <you@example.com>",
    about = "A modular and scalable CLI app",
	disable_help_flag = true
)]
#[command(group(ArgGroup::new("target").required(true).args(&["host", "subnet", "range"])))]
struct CommandLine {
    #[arg(short, long, group = "target")]
    host: Option<String>,

    #[arg(short, long, group = "target")]
    subnet: Option<String>,

    #[arg(short, long, group = "target")]
    range: Option<String>,

    #[arg(long)]
    user: Option<String>,

    #[arg(long)]
    pass: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Model,
    Check,
}

fn main() {
    let cmd = CommandLine::parse();
	let auth = match(&cmd.user, &cmd.pass) {
        (Some(user), Some(pass))  => Some(Auth::new(user, pass)),
        (None, None) => None,
        _ => panic!("Both --user and --pass must be provided together"),
    };

	match (cmd.host, cmd.subnet, cmd.range) {
		(Some(host), None, None) => t_host(&host, auth.as_ref()),
		(None, Some(subnet), None) => t_subnet(&subnet, auth.as_ref()),
		(None, None, Some(range)) => t_range(&range, auth.as_ref()),
		_ => unreachable!(),
	}

    match &cmd.command {
        Commands::Model=> model(),
        Commands::Check=> version(),
    }
}

