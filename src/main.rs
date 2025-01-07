use std::thread::spawn;
use std::{fs, net::SocketAddr, path::PathBuf, process::exit};

use crate::CompileArgs;
use clap::Parser;
use compile::get_compiler_actor;
use reflexo_typst::error::prelude::ZResult;
use tokio::runtime::Builder;
use typst_hugo::*;
use warp::Filter;
use warp::http::Method;

fn main() {
    env_logger::builder()
        .filter_module("typst", log::LevelFilter::Warn)
        .filter_module("reflexo", log::LevelFilter::Info)
        .filter_module("typst_hugo",log::LevelFilter::Debug)
	
        .init();

    let opts = Opts::parse();

    
    let fut = async {
	match opts.sub {
	    Some(Subcommands::Compile(args)) => {
		build(args).await
	    }
	    Some(Subcommands::Serve(args)) => {
		if !args.no_build {
		    tokio::spawn(build(args.compile.clone()));
		}
		serve(args).await
	    }
	    None => build(CompileArgs::default()).await,
	};
	()
    };

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(fut);
	
}


fn prepare_asset(asset_dir: &PathBuf) {
    let internal = asset_dir.join("internal");
    if !internal.exists() {
	fs::create_dir_all(internal.clone()).expect("create asset directory failed");
    }    
    fs::write(
        internal.join("typst_ts_renderer_bg.wasm"),
        include_bytes!("../assets/artifacts/typst_ts_renderer_bg.wasm"),
    ).unwrap();
    fs::write(
        internal.join("svg_utils.js"),
        include_bytes!("../assets/artifacts/svg_utils.cjs"),
    ).unwrap();
    fs::write(
        internal.join("book.mjs"),
        include_bytes!("../assets/artifacts/book.mjs"),
    ).unwrap();
    let css_dir = asset_dir.join("themes").join("css");
    if !css_dir.exists() {
	fs::create_dir_all(css_dir.clone()).expect("create asset directory failed");
    }
    fs::write(
        css_dir.join("general.css"),
        include_bytes!("../themes/css/general.css"),
    ).unwrap();
    

}
async fn build(args: CompileArgs) {
    prepare_asset(&args.asset_dir);
    let actor = get_compiler_actor(args).unwrap_or_else(|x| {
	log::error!("{}", x);
	exit(1);
    });

    actor.run().await;
}



async fn serve(args: ServeArgs) {

    let http_addr: SocketAddr = args
        .addr
        .clone()
        .parse()
	.expect("Can't parse address");

    log::info!("Server listening on http://{:?}", http_addr);

    let server = warp::serve({
        let cors =
            warp::cors().allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::HEAD]);

        let dev = warp::path("dev").and(warp::fs::dir(""));

        dev.or(warp::fs::dir(args.compile.asset_dir))
            .with(cors)
            .with(warp::compression::gzip())
    });

    server.run(http_addr).await;

    exit(0);
}
