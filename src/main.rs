use std::{
    io,
    process::{Command, Stdio},
};

use cargo_metadata::camino::Utf8PathBuf;
use clap::Parser;
use itertools::Itertools;
use miette::IntoDiagnostic;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config_file: Option<Utf8PathBuf>,
    #[arg(short = 'f', long)]
    config: Option<String>,
    rest: Vec<String>,
}

fn main() -> miette::Result<()> {
    let Args {
        mut config_file,
        config,
        mut rest,
    } = Args::parse();

    let mut config_contents = Vec::new();

    #[cfg(feature = "auto-resolve")]
    let falled_back = config_file.is_none();

    #[cfg(feature = "auto-resolve")]
    if config_file.is_none() {
        let mut workspace = cargo_metadata::MetadataCommand::new()
            .exec()
            .into_diagnostic()?
            .workspace_root;
        workspace.push("rustfmt.toml");
        config_file = Some(workspace);
    }

    if let Some(config_path) = config_file {
        match std::fs::read_to_string(config_path) {
            Ok(content) => {
                let content = content.parse::<toml::Table>().into_diagnostic()?;

                for (key, val) in content {
                    let str_val = match val {
                        toml::Value::String(s) => s,
                        toml::Value::Integer(i) => i.to_string(),
                        toml::Value::Float(f) => f.to_string(),
                        toml::Value::Boolean(b) => b.to_string(),
                        toml::Value::Datetime(d) => d.to_string(),
                        toml::Value::Array(_) => todo!(),
                        toml::Value::Table(_) => todo!(),
                    };
                    let line = format!("{key}={str_val}");
                    if !config_contents.contains(&line) {
                        config_contents.push(line);
                    }
                }
            }
            #[cfg(feature = "auto-resolve")]
            Err(_) if falled_back => {}
            Err(e) => Err(e).into_diagnostic()?,
        }
    }

    if let Some(content) = config {
        let lines = content.split(',');

        for line in lines {
            let line = line.to_owned();
            if !config_contents.contains(&line) {
                config_contents.push(line);
            }
        }
    }

    let config_index = rest
        .iter()
        .find_position(|a| *a == "--config")
        .map(|(a, _)| a);
    let config_value = config_index.and_then(|index| rest.get(index + 1));

    if let Some(content) = config_value {
        let lines = content.split(',');

        for line in lines {
            let line = line.to_owned();
            if !config_contents.contains(&line) {
                config_contents.push(line);
            }
        }
    }

    let has_value = config_value.is_some();

    if let Some(index) = config_index {
        rest.remove(index);
        if has_value {
            rest.remove(index);
        }
    }

    let config_options =
        itertools::Itertools::intersperse(config_contents.into_iter(), ",".to_owned())
            .collect::<String>();

    if rest.is_empty() {
        rest.extend(
            ["cargo", "fmt", "--all", "--check", "--"]
                .into_iter()
                .map(ToOwned::to_owned),
        );
    }

    let empty_config_loc = "./rustfmt_unstable_empty_config_file_abcbebdbebeb.toml";

    if let Some(index) = config_index {
        rest.insert(index, "--config".to_string());
        rest.insert(index + 1, config_options);
        rest.insert(index + 2, "--config-path".to_string());
        rest.insert(index + 3, empty_config_loc.to_string());
    } else {
        rest.push("--config".to_string());
        rest.push(config_options);
        rest.push("--config-path".to_string());
        rest.push(empty_config_loc.to_string());
    }

    std::fs::File::create(empty_config_loc).into_diagnostic()?;

    let (program, args) = rest
        .split_first()
        .expect("This shouldn't happen as we push something when it is empty above");
    let mut command = Command::new(program);
    let status = command
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(io::stdout())
        .stderr(io::stderr())
        .spawn()
        .into_diagnostic()?
        .wait()
        .into_diagnostic()?;

    std::fs::remove_file(empty_config_loc).into_diagnostic()?;

    if let Some(code) = status.code() {
        std::process::exit(code);
    }

    Ok(())
}
