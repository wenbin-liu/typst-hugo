pub mod compile;

use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use serde::Serialize;

#[derive(Default, Debug, Clone, Parser, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum DarkModeAvailable {
    #[default]
    Meme,
    Blowfish,
}


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

    #[clap(long, short, default_value = "meme")]
    pub darkmode_callback:DarkModeAvailable,
        
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

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Template options")]
pub struct TemplateArgs {
    #[clap(default_value = "./")]
    pub path: PathBuf,
}

#[derive(Debug, Parser)]
#[clap(name = "typst-hugo", version = "0.1.0")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Option<Subcommands>,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Compile tyspt to html page for hugo",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(about = "Build html")]
    Compile(CompileArgs),
    #[clap(about = "Serve html")]
    Serve(ServeArgs),

    #[clap(about = "Generate typst template", name = "template")]
    TypstTemplate(TemplateArgs),
}
