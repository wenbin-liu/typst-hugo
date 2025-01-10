pub mod compile;

use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile options")]
pub struct CompileArgs {
    #[clap(required = true)]
    pub entry: PathBuf,

    #[clap(default_value = "./", short, long)]
    pub root: PathBuf,

    #[clap(long, default_value = "./")]
    pub html_dir: PathBuf,

    #[clap(long, short, default_value = "./")]
    pub asset_dir: PathBuf,

    #[clap(long, default_value = "/")]
    pub path_to_root: String,

    #[clap(long, default_values=vec!["light", "dark"])]
    pub theme: Vec<String>,

    #[clap(long)]
    pub watch: bool,

    #[clap(long, default_value = "true")]
    pub front_matter: bool,

    #[clap(long, default_value = "false")]
    pub no_assets: bool, 
    
    #[clap(
        long = "font-path",
        env = "TYPST_FONT_PATHS", 
        value_name = "DIR",
        action = ArgAction::Append,
    )]
    pub font_paths: Vec<PathBuf>,
}

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile options")]
pub struct ServeArgs {
    /// arguments for compile setting.
    #[clap(flatten)]
    pub compile: CompileArgs,

    /// Do not build the book before serving.
    #[clap(long)]
    pub no_build: bool,

    /// Listen address.
    #[clap(long, default_value = "127.0.0.1:25520")]
    pub addr: String,
}


#[derive(Debug, Parser)]
#[clap(name = "typst-hugo", version = "0.1.0")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Option<Subcommands>,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Compile tyspt to html page",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(about = "build html")]
    Compile(CompileArgs),
    #[clap(about = "serve book.")]
    Serve(ServeArgs),
}


