use clap::Parser;

mod config;
mod parser;
mod tools;

#[derive(Parser)]
#[command(version, about)]
//#[clap(color = ColorChoice::Always)]
struct Opts {
    #[arg(short, long)]
    package: Option<std::path::PathBuf>,
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let opts = Opts::parse();

    simplelog::TermLogger::init(
        opts.verbose.log_level_filter(),
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("Error setting up simplelog.");

    let package_path = if let Some(path) = opts.package.as_ref() {
        std::borrow::Cow::Borrowed(path)
    } else {
        std::borrow::Cow::Owned({
            let current_path = std::env::current_dir().unwrap_or_else(|e| {
                log::error!(
                    "Cannot obtain current path. Pass the `package` argument manually.\n[{e}]"
                );
                std::process::exit(0);
            });
            log::info!("Using `{}` as the package path.", current_path.display());
            current_path
        })
    };

    log::trace!("Parsing the `typst.toml` file.");
    // read typst.toml first, before parsing the other arguments.
    let config = parser::parse_typst_toml(&package_path).unwrap_or_else(|e| {
        log::error!("{e:?}");
        std::process::exit(0);
    });

    let output_path = if let Some(path) = opts.output.as_ref() {
        path.join(&config.name).join(&config.version)
    } else {
        directories::BaseDirs::new()
            .expect("Could not access typst packages cache. Pass `output` argument manually.")
            .cache_dir()
            .join("typst")
            .join("packages")
            .join("local")
            .join(&config.name)
            .join(&config.version)
    };
    log::info!("Using `{}` as the output path", output_path.display());

    tools::tester::compile_tests(&package_path).unwrap_or_else(|e| {
        log::error!("{e:?}");
        std::process::exit(0)
    });

    tools::compiler::compile_files(&config, &package_path).unwrap_or_else(|e| {
        log::error!("{e:?}");
        std::process::exit(0)
    });

    tools::packer::package_into(&config, &package_path, &output_path, opts.output.is_none())
        .unwrap_or_else(|e| {
            log::error!("{e:?}");
            std::process::exit(0)
        });

    if let Some(script) = &config.typpkg_config.postscript {
        log::trace!("Starting the postscript.");
        std::process::Command::new(script.command.clone())
            .args(script.arguments.iter())
            .current_dir(&output_path)
            .status()
            .unwrap_or_else(|e| {
                log::error!("Error executing the postscript. {e}");
                std::process::exit(0)
            });
    }

    println!(
        "Project released to {}.{}
- release it to 'https://github.com/typst/packages'.",
        output_path.display(),
        if opts.output.is_none() {
            format!(
                "\n- test and/or use via `#import \"@local/{}:{}\"` it, or",
                config.name, config.version
            )
        } else {
            "".to_owned()
        }
    );
}
