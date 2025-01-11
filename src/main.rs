use std::{fs, net::SocketAddr, path::PathBuf, process::exit};

use crate::CompileArgs;
use clap::Parser;
use compile::get_compiler_actor;
use tokio::runtime::Builder;
use typst_hugo::*;
use warp::http::Method;
use warp::Filter;

fn main() {
    env_logger::builder()
        .filter_module("typst", log::LevelFilter::Warn)
        .filter_module("reflexo", log::LevelFilter::Info)
        .filter_module("typst_hugo", log::LevelFilter::Debug)
        .init();

    let opts = Opts::parse();

    let fut = async {
        match opts.sub {
            Some(Subcommands::Compile(args)) => build(args).await,
            Some(Subcommands::Serve(args)) => {
                if !args.no_build {
                    tokio::spawn(build(args.compile.clone()));
                }
                serve(args).await
            }
            Some(Subcommands::TypstTemplate(args)) => generate_typst_template(args),
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
    )
    .unwrap();
    fs::write(
        internal.join("svg_utils.js"),
        include_bytes!("../assets/artifacts/svg_utils.cjs"),
    )
    .unwrap();
    fs::write(
        internal.join("book.mjs"),
        include_bytes!("../assets/artifacts/book.mjs"),
    )
    .unwrap();
    let css_dir = asset_dir.join("themes").join("css");
    if !css_dir.exists() {
        fs::create_dir_all(css_dir.clone()).expect("create asset directory failed");
    }
    fs::write(
        css_dir.join("general.css"),
        include_bytes!("../themes/css/general.css"),
    )
    .unwrap();
}
async fn build(args: CompileArgs) {
    if !args.no_assets {
        prepare_asset(&args.asset_dir);
    }

    let actor = get_compiler_actor(args).unwrap_or_else(|x| {
        log::error!("{}", x);
        exit(1);
    });

    actor.run().await;
}

async fn serve(args: ServeArgs) {
    let http_addr: SocketAddr = args.addr.clone().parse().expect("Can't parse address");

    log::info!(
        "Server listening on http://{:?}/{}.html",
        http_addr,
        args.compile.entry.file_stem().unwrap().to_str().unwrap()
    );

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

fn generate_typst_template(args: TemplateArgs) {
    let path = args.path.join("template");
    if !path.exists() {
        fs::create_dir_all(path.clone()).unwrap();
    }
    let path = path.join("hugo_template.typ");
    if path.exists() {
        log::error!("hugo_template.typ exists!");
        exit(1);
    }
    fs::write(path, include_bytes!("../template/hugo_template.typ")).unwrap();

    let path = args.path.join("main.typ");
    if path.exists() {
        log::error!("main.typ exists!");
        exit(1);
    }

    let typst_main = r##"#import "template/hugo_template.typ": project
#show: project.with(
    title: [Types Template],
    date: datetime.today(),
    tags:("Typst","Rust"),
    categories:("Computers",)
)

= Section 1
Hello Typst
"##;
    fs::write(path, typst_main).unwrap();
}
