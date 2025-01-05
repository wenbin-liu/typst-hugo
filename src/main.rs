use std::process::exit;

use crate::compile::CompileArgs;
use clap::{Args, Command, FromArgMatches, Parser};
use compile::get_compiler_actor;
use tokio::runtime::Builder;
use typst_hugo::*;


async fn build(args: CompileArgs) {
    let actor = get_compiler_actor(args).unwrap_or_else(|x| {
	log::error!("{}", x);
	exit(1);
    });

    actor.run().await;
}
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let opts = Opts::parse();

    
    let fut = match opts.sub {
	Some(Subcommands::Compile(args)) => {
	    build(args)
	}
	None => build(CompileArgs::default()),
    };

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(fut);
	
}


