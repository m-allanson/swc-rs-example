use std::sync::Arc;

use swc::{
    self,
    config::{
        Config, GlobalPassOption, JscConfig, JscTarget, OptimizerConfig, Options, SourceMapsConfig,
        TransformConfig,
    },
};
use swc_common::{
    // chain,
    errors::{ColorConfig, Handler},
    FileName,
    SourceMap,
};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms::react;

pub fn compile_js_for_browser(
    source: String,
    filename: String,
) -> String {
    let opts = &get_opts();
    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));

    let compiler = swc::Compiler::new(cm.clone());

    let fm = cm.new_source_file(FileName::Custom(filename.clone()), source);

    let parsed_program =
        compiler.parse_js(fm, &handler, JscTarget::Es2020, get_syntax(), true, true);
    let built_config =
        compiler.config_for_file(&handler, opts, &FileName::Custom(filename.clone()));

    let result = compiler.transform(
        &handler,
        parsed_program.unwrap(),
        false,
        built_config.unwrap().unwrap().pass,
    );

    dbg!(&result);

    let output = compiler.print(
        &result,
        Some(&filename),
        None,
        JscTarget::Es2020,
        SourceMapsConfig::default(),
        &[],
        None,
        false,
        Some(true.into())
    );

    output.unwrap().code
}

// pub fn compile_js_for_server(source: String, filename: String, npm_bin_dir: PathBuf) -> String {
//     let opts = &get_opts();

//     let cm = Arc::<SourceMap>::default();
//     let handler = Arc::new(Handler::with_tty_emitter(
//         ColorConfig::Auto,
//         true,
//         false,
//         Some(cm.clone()),
//     ));

//     let compiler = swc::Compiler::new(cm.clone());

//     let fm = cm.new_source_file(FileName::Custom(filename.clone()), source);

//     let parsed_program =
//         compiler.parse_js(fm, &handler, JscTarget::Es2020, get_syntax(), true, true);
//     let built_config =
//         compiler.config_for_file(&handler, opts, &FileName::Custom(filename.clone()));

//     let result = compiler.transform(
//         &handler,
//         parsed_program.unwrap(),
//         false,
//         built_config.unwrap().unwrap().pass,
//     );

//     let output = compiler.print(
//         &result,
//         Some(&filename),
//         None,
//         JscTarget::Es2020,
//         SourceMapsConfig::default(),
//         None,
//         false,
//         None,
//         Some(swc::config::util::BoolOrObject::Bool(true))
//     );

//     output.unwrap().code
// }

fn get_opts() -> Options {
    Options {
        is_module: true,
        config: Config {
            jsc: JscConfig {
                target: Some(JscTarget::Es2020),
                syntax: Some(Syntax::Es(EsConfig {
                    jsx: true,
                    nullish_coalescing: true,
                    optional_chaining: true,
                    dynamic_import: true,
                    ..Default::default()
                })),
                transform: Some(TransformConfig {
                    react: react::Options {
                        pragma: "h".to_string(),
                        pragma_frag: "Fragment".to_string(),
                        use_builtins: true,
                        ..Default::default()
                    },
                    optimizer: Some(OptimizerConfig {
                        globals: Some(GlobalPassOption {
                            envs: std::env::vars()
                                .filter_map(|(k, _)| {
                                    if k.starts_with("TOAST_") {
                                        Some(k)
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

fn get_syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        nullish_coalescing: true,
        optional_chaining: true,
        dynamic_import: true,
        ..Default::default()
    })
}


fn main() {
    println!("Hello, world!");
    
    dbg!("const thing = 3;\n/* now what */");

    let result = compile_js_for_browser(
        "const thing = 3;\n/* now what */".to_string(), 
        "example.js".to_string()
    );

    // Will this show the transformed source?
    dbg!(result);
}
