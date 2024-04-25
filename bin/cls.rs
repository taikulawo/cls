use std::{env, process::exit};

use clap::{command, Parser};
use cls::clash::{self, generate};
use cls::commands::Subcommands;
use env_logger::{Builder, Target};
use log::debug;

use cls::config::CLASH_CONFIG_FOLDER_PATH;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    sub_commands: Subcommands,
    #[arg(long, help = "verbose log with child process log")]
    verbose: bool,
}
fn main() {
    let cli = Cli::try_parse();
    let res: anyhow::Result<()> = match cli {
        Err(err) => {
            if matches!(
                err.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            ) {
                // 输出--help
                err.print().map_err(anyhow::Error::from)
            } else {
                Ok(())
            }
        }
        Ok(cli) => {
            let uid = unsafe { libc::getuid() };
            if uid != 0 && !matches!(cli.sub_commands, Subcommands::Generate(..)) {
                eprintln!("run under root privileges :)");
                exit(0);
            }
            let mut level = "info".to_string();
            if cli.verbose {
                level = "debug".into();
            }
            if let Ok(v) = env::var("RUST_LOG") {
                level = v;
            }
            env::set_var("RUST_LOG", format!("cls={}", level));
            let mut builder: Builder = Builder::from_default_env();
            builder
                .target(Target::Stdout)
                .format_timestamp(None)
                .format_level(true)
                .format_target(true)
                .write_style(env_logger::WriteStyle::Auto);
            builder.init();
            match cli.sub_commands {
                Subcommands::Install(install) => {
                    debug!(
                        "cls config directory location {}",
                        CLASH_CONFIG_FOLDER_PATH.display()
                    );
                    clash::install_all(install.tun, install.update)
                }
                Subcommands::Clean(..) => clash::clean_all(),
                Subcommands::Server(server) => clash::start_proxy_in_foregroud(server),
                Subcommands::Generate(args) => generate(args),
            }
        }
    };
    if let Err(err) = res {
        eprintln!("{:?}", err)
    }
}
