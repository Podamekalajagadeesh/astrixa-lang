// STEP 50: ASTRIXA CLI - Main Entry Point
// Professional developer toolchain for ASTRIXA language

use clap::{Command, Arg, ArgMatches};
use colored::*;
use std::process;

mod commands;
mod config;
mod templates;

use commands::{new, build, run, add};

fn main() {
    let matches = Command::new("astrixa")
        .version("0.1.0")
        .author("ASTRIXA Team")
        .about("ASTRIXA Language Toolchain - Build, run, and manage ASTRIXA projects")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .about("Create a new ASTRIXA project")
                .arg(
                    Arg::new("name")
                        .help("Name of the project")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("template")
                        .long("template")
                        .short('t')
                        .help("Project template (default, lib, web3, ai)")
                        .default_value("default")
                )
        )
        .subcommand(
            Command::new("build")
                .about("Build the current project")
                .arg(
                    Arg::new("release")
                        .long("release")
                        .short('r')
                        .help("Build in release mode with optimizations")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("Output file path")
                        .value_name("FILE")
                )
        )
        .subcommand(
            Command::new("run")
                .about("Build and run the current project")
                .arg(
                    Arg::new("release")
                        .long("release")
                        .short('r')
                        .help("Run in release mode")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("add")
                .about("Add a package dependency")
                .arg(
                    Arg::new("package")
                        .help("Package name")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("version")
                        .long("version")
                        .short('v')
                        .help("Package version")
                        .default_value("latest")
                )
        )
        .subcommand(
            Command::new("init")
                .about("Initialize ASTRIXA project in current directory")
        )
        .subcommand(
            Command::new("check")
                .about("Check the project for errors without building")
        )
        .subcommand(
            Command::new("clean")
                .about("Remove build artifacts")
        )
        .get_matches();

    let result = match matches.subcommand() {
        Some(("new", sub_matches)) => handle_new(sub_matches),
        Some(("build", sub_matches)) => handle_build(sub_matches),
        Some(("run", sub_matches)) => handle_run(sub_matches),
        Some(("add", sub_matches)) => handle_add(sub_matches),
        Some(("init", _)) => handle_init(),
        Some(("check", _)) => handle_check(),
        Some(("clean", _)) => handle_clean(),
        _ => {
            eprintln!("{}", "Unknown command".red());
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn handle_new(matches: &ArgMatches) -> Result<(), String> {
    let name = matches.get_one::<String>("name").unwrap();
    let template = matches.get_one::<String>("template").unwrap();
    
    new::create_project(name, template)
}

fn handle_build(matches: &ArgMatches) -> Result<(), String> {
    let release = matches.get_flag("release");
    let output = matches.get_one::<String>("output");
    
    build::build_project(release, output)
}

fn handle_run(matches: &ArgMatches) -> Result<(), String> {
    let release = matches.get_flag("release");
    
    run::run_project(release)
}

fn handle_add(matches: &ArgMatches) -> Result<(), String> {
    let package = matches.get_one::<String>("package").unwrap();
    let version = matches.get_one::<String>("version").unwrap();
    
    add::add_package(package, version)
}

fn handle_init() -> Result<(), String> {
    new::init_project()
}

fn handle_check() -> Result<(), String> {
    build::check_project()
}

fn handle_clean() -> Result<(), String> {
    build::clean_project()
}
