use std::borrow::Cow;
use std::{path::PathBuf, sync::Arc};

use clap::{ArgAction, Parser};

use log::debug;
use reflexo_typst::config::entry::{EntryOpts, MEMORY_MAIN_ENTRY};
use reflexo_typst::config::CompileOpts;
use reflexo_typst::features::{FeatureSet, DIAG_FMT_FEATURE};
use reflexo_typst::{exporter_builtins::GroupExporter, path::PathClean};
use reflexo_typst::{
    CompilationHandle, CompileActor, CompileDriver, CompileExporter, CompileServerOpts, CompileStarter, CompiledArtifact, CompilerFeat, ConsoleDiagReporter, DiagnosticFormat, DynExporter, DynamicLayoutCompiler, EntryManager, EntryReader, GenericExporter, PureCompiler, ShadowApi, SystemCompilerFeat, TypstSystemUniverse, TypstSystemWorld
};
use tokio::sync::mpsc;

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile options")]
pub struct CompileArgs {
    #[clap(required = true)]
    pub entry: PathBuf,

    #[clap(default_value = "./", short, long)]
    pub root_dir: PathBuf,

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
    
    #[clap(
        long = "font-path",
        env = "TYPST_FONT_PATHS", 
        value_name = "DIR",
        action = ArgAction::Append,
    )]
    pub font_paths: Vec<PathBuf>,

    /// Specify a filter to only load files with a specific extension.
    #[clap(long, default_value = "^(player.bilibili.com)$")]
    pub allowed_url_source: Option<String>,
}


type SystemDynamicLayoutCompiler =
    DynamicLayoutCompiler<SystemCompilerFeat, PureCompiler<TypstSystemWorld>>;

pub struct CompileHandler<F: CompilerFeat> {
    exporter: GroupExporter<CompiledArtifact<F>>,
}

impl<F: CompilerFeat + 'static> CompilationHandle<F> for CompileHandler<F> {
    fn status(&self, _revision: usize, _rep: reflexo_typst::CompileReport) {}

    fn notify_compile(
        &self,
        compiled: &reflexo_typst::CompiledArtifact<F>,
        rep: reflexo_typst::CompileReport,
    ) {
        use reflexo_typst::Exporter;
        if let reflexo_typst::CompileReport::CompileSuccess(t, ..) = rep {
            let curr = reflexo_typst::time::now();
            let errs = self
                .exporter
                .export(compiled.world.as_ref(), Arc::new(compiled.clone()));
            let elapsed = curr.elapsed().unwrap_or_default();	    
            if let Err(errs) = errs {

                let rep = reflexo_typst::CompileReport::ExportError(t, errs, elapsed);
                let _ = ConsoleDiagReporter::default().export(
                    compiled.world.as_ref(),
                    Arc::new((compiled.env.features.clone(), rep.clone())),
                );
            } else {
		// log::info!("Compiled in {:?}", elapsed);
	    }
        }
    }
}

fn clean_path(args: CompileArgs) -> CompileArgs{
    let mut args = args;
    args.root_dir = args.root_dir.clean();
    args.entry = args.entry.clean();

    args.root_dir = if args.root_dir.is_absolute() {
	args.root_dir
    } else {
        let cwd = std::env::current_dir().expect("Can't get the pwd");
        cwd.join(args.root_dir).clean()
    };

    args.entry = if args.entry.is_absolute() {
	args.entry
    } else {
        let cwd = std::env::current_dir().expect("Can't get the pwd");
        cwd.join(args.entry)
    };    

    args
}
 
pub fn get_compiler_actor(args:CompileArgs) -> Result<CompileActor<SystemCompilerFeat>, String> {
    debug!("font-path:{:?}", args.font_paths);
    let args = clean_path(args);
    let verse = TypstSystemUniverse::new(CompileOpts {
	entry: EntryOpts::new_workspace(args.root_dir.clone()),
	font_paths: args.font_paths.clone(),
        with_embedded_fonts: typst_assets::fonts().map(Cow::Borrowed).collect(),
	..CompileOpts::default()
    }).map_err(|e| e.to_string())?;

    let mut output_name = args.entry.clone();
    let output_name = output_name.file_stem().ok_or( "Invalid entry file")?;
    
    let verse = verse.with_entry_file(args.entry);



    let (intr_tx, intr_rx) = mpsc::unbounded_channel();

    let mut exporters: Vec<DynExporter<CompiledArtifact<SystemCompilerFeat>>> = vec![];

    let output_file = if args.asset_dir.is_dir() {
	args.asset_dir.join(output_name)
    } else {
	args.asset_dir
    };

    for theme in args.theme {
	let mut dyn_driver = DynamicLayoutCompiler::new(std::marker::PhantomData, output_file.clone());
	dyn_driver.set_target("web-".to_string() + &theme);
	dyn_driver.set_extension(theme + ".multi.sir.in");
	exporters.push(Box::new(CompileStarter::new(dyn_driver)));
    }


    let handle = Arc::new(CompileHandler {
        exporter: GroupExporter::new(exporters),
    });

    let feature_set =
        FeatureSet::default().configure(&DIAG_FMT_FEATURE, DiagnosticFormat::Human,);

    let actor = CompileActor::new_with(
        verse,
        intr_tx,
        intr_rx,
        CompileServerOpts {
            compile_handle: handle,
	    feature_set,	    
            ..Default::default()
        },
    ).with_watch(args.watch);

    
    Ok(actor)
}
