use clap::{Args, Subcommand};
#[derive(Args, Clone, Default)]
pub struct InstallCommand {
    #[arg(long, help = "tun mode")]
    pub tun: bool,

    #[arg(long, help = "update latest config.yaml from remote before install")]
    pub update: bool,
}

#[derive(Args, Clone, Default)]
pub struct ServerCommand {
    #[arg(long, help = "tun mode")]
    pub tun: bool,
    #[arg(
        long,
        help = "update latest config.yaml from remote before start server"
    )]
    pub update: bool,
}

#[derive(Args, Clone, Default)]
pub struct CleanCommand {}

#[derive(Args, Clone, Default)]
pub struct GenerateCommand {
    #[arg(long, help = "tun mode")]
    pub tun: bool,
}

#[derive(Args, Clone, Default)]
pub struct UpdateCommand {}

#[derive(Subcommand)]
pub enum Subcommands {
    #[command(visible_alias = "i", about = "install cls, proxychains etc")]
    Install(InstallCommand),
    #[command(about = "start proxy server in foregroud")]
    Server(ServerCommand),
    #[command(about = "clean previous installation")]
    Clean(CleanCommand),
    #[command(about = "read clash config from stdin, output the final config.yaml")]
    Generate(GenerateCommand),
}
