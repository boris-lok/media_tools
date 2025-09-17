use clap::{command, value_parser, Arg, Command};
use std::path::PathBuf;
use video::concat;

fn main() {
    let command = command_builder();
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("concat", args)) => {
            let input = args.get_one::<PathBuf>("folder");
            if input.is_none() {
                eprintln!("No input folder was provided");
            }

            let output = args.get_one::<PathBuf>("output");
            if output.is_none() {
                eprintln!("No output file was provided");
            }

            let ext = args.get_one::<String>("ext");
            if ext.is_none() {
                eprintln!("No extension was provided");
            }

            let prefix = args.get_one::<String>("prefix");
            if prefix.is_none() {
                eprintln!("No prefix was provided");
            }

            if let Err(e) = concat(
                input.unwrap().as_path(),
                prefix.unwrap(),
                ext.unwrap(),
                output.unwrap().as_path(),
            ) {
                eprintln!("{:?}", e);
            }
        }
        _ => eprintln!("No subcommand was provided"),
    }
}

fn command_builder() -> Command {
    command!()
        .version("v0.1.0")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([Command::new("concat")
            .about("Concat multiple video files into one.")
            .args([
                Arg::new("folder")
                    .short('f')
                    .long("folder")
                    .required(true)
                    .help("The folder contains the video files.")
                    .value_parser(value_parser!(PathBuf)),
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .required(true)
                    .help("The output file path")
                    .value_parser(value_parser!(PathBuf)),
                Arg::new("ext")
                    .long("ext")
                    .required(true)
                    .help("The video files' extension")
                    .value_parser(value_parser!(String)),
                Arg::new("prefix")
                    .short('p')
                    .long("prefix")
                    .required(true)
                    .help("The video files' prefix")
                    .value_parser(value_parser!(String)),
            ])])
}
