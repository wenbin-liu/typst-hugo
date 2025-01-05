pub mod compile;

use compile::CompileArgs;

use clap::{Parser, Subcommand};


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
    // #[clap(about = "serve book.")]
    // Serve(()),
}


