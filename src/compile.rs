use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::slice::SliceIndex;
use std::str::FromStr;
use std::{path::PathBuf, sync::Arc};

use chrono::{Date, DateTime, Local};
use clap::{ArgAction, Parser};

use handlebars::Handlebars;
use log::debug;
use reflexo_typst::config::entry::EntryOpts;
use reflexo_typst::config::CompileOpts;
use reflexo_typst::features::{CompileFeature, FeatureSet, DIAG_FMT_FEATURE};
use reflexo_typst::{exporter_builtins::GroupExporter, path::PathClean};
use reflexo_typst::{
    CompilationHandle, CompileActor, CompileServerOpts, CompileStarter, CompiledArtifact, CompilerFeat, ConsoleDiagReporter, DiagnosticFormat, DynExporter, DynamicLayoutCompiler, GenericExporter, SystemCompilerFeat, TypstSystemUniverse
};
use tokio::sync::mpsc;
use typst::foundations::{AutoValue, Label, Selector};

use crate::CompileArgs;




pub struct CompileHandler<F: CompilerFeat> {
    compile_args: CompileArgs,
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
		render_html(compiled, &self.compile_args);	
	    }
        }
    }
}

fn render_html<F: CompilerFeat>(compiled: &CompiledArtifact<F>, compile_args: &CompileArgs)  {
    let info = compiled.doc.as_ref().unwrap().info.clone();
    log::debug!("Compiled doc info:{:?}", info);
    let meta = compiled.doc.as_ref().unwrap().introspector.query_first(&Selector::Label(Label::new("typst_hugo_0xbafe783")));
    log::debug!("Meta Data:{:?}", meta);
    let mut res = serde_json::to_value(&meta)
        .unwrap();
    let mut res = res["value"].take();
    res["title"] = info.title.unwrap_or_default().to_string().into();
    res["author"] = info.author.iter().map(|x|x.to_string()).collect();
    let typst_date = info.date.unwrap_or_default().map(
	|x|x.display(typst::foundations::Smart::Auto).unwrap_or_default().to_string()).unwrap_or_default();
    let typst_date = if typst_date == "" {
	Local::now().date_naive().to_string()
    } else {
	typst_date
    };
    res["date"] = typst_date.into();
    res["path_to_root"] = compile_args.path_to_root.clone().into();
    res["rel_data_path"] = get_sir_name(&compile_args).into();
    res["renderer_module"] = "internal/typst_ts_renderer_bg.wasm".into();
    

    let mut hb = Handlebars::new();
    hb.register_template_string("index",
				String::from_utf8(include_bytes!("../themes/index.hbs").to_vec()).unwrap()).unwrap();
    
    let html = hb.render("index",&res).unwrap();

    let html_path = if compile_args.html_dir.is_dir() {
	let mut name = compile_args.html_dir.join(compile_args.entry.file_stem().unwrap());
	name.set_extension("html");
	name
    } else {
	compile_args.html_dir.clone()
    };
    
    fs::write(html_path, html).unwrap();
    
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

fn get_sir_name(args: &CompileArgs) -> String {
    let output_name = args.entry.clone();
    let output_name = output_name.file_stem().expect( "Invalid entry file");
    

    let sir_filename = if args.asset_dir.is_dir() {
	output_name
    } else {
	args.asset_dir.file_stem().expect("Invalid asset dir")
    };

    String::from(sir_filename.to_str().unwrap())
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

    let output_name = args.entry.clone();
    let output_name = output_name.file_stem().ok_or( "Invalid entry file")?;
    
    let verse = verse.with_entry_file(args.entry.clone());



    let (intr_tx, intr_rx) = mpsc::unbounded_channel();

    let mut exporters: Vec<DynExporter<CompiledArtifact<SystemCompilerFeat>>> = vec![];

    let output_file = if args.asset_dir.is_dir() {
	args.asset_dir.join(output_name)
    } else {
	args.asset_dir.clone()
    };

    for theme in &args.theme {
	let mut dyn_driver = DynamicLayoutCompiler::new(std::marker::PhantomData, output_file.clone());
	dyn_driver.set_target("web-".to_string() + &theme);
	dyn_driver.set_extension(theme.to_string() + ".multi.sir.in");
	exporters.push(Box::new(CompileStarter::new(dyn_driver)));
    }


    let watch = args.watch;
    let handle = Arc::new(CompileHandler {
	compile_args: args,
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
    ).with_watch(watch);

    
    Ok(actor)
}
