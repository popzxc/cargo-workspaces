use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::{AppSettings, Clap};
use console::Term;

mod changed;
mod create;
mod exec;
mod list;
mod publish;
mod version;

mod utils;

#[derive(Debug, Clap)]
enum Subcommand {
    // TODO: add
    #[clap(alias = "ls")]
    List(list::List),
    Changed(changed::Changed),
    Version(version::Version),
    Publish(publish::Publish),
    Exec(exec::Exec),
    Create(create::Create),
}

#[derive(Debug, Clap)]
#[clap(
    version,
    global_setting(AppSettings::VersionlessSubcommands),
    replace("la", &["list", "-a"]),
    replace("ll", &["list", "-l"])
)]
struct Opt {
    /// Path to workspace Cargo.toml
    #[clap(long, value_name = "path")]
    manifest_path: Option<String>,

    /// Verbose mode
    #[clap(short)]
    verbose: bool,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, Clap)]
#[clap(
    name = "cargo-workspaces",
    bin_name = "cargo",
    version,
    global_setting(AppSettings::ColoredHelp)
)]
enum Cargo {
    #[clap(alias = "ws")]
    Workspaces(Opt),
}

fn main() {
    let Cargo::Workspaces(opt) = Cargo::parse();

    if opt.verbose {
        utils::error::set_debug();
    }

    let mut cmd = MetadataCommand::new();

    cmd.features(CargoOpt::AllFeatures);
    cmd.no_deps();

    if let Some(path) = opt.manifest_path {
        cmd.manifest_path(path);
    }

    let metadata = cmd.exec().unwrap();
    let stdout = Term::stdout();
    let stderr = Term::stderr();

    let err = match opt.subcommand {
        Subcommand::List(x) => x.run(metadata, &stdout, &stderr),
        Subcommand::Changed(x) => x.run(metadata, &stdout, &stderr),
        Subcommand::Version(x) => x.run(metadata, &stdout, &stderr),
        Subcommand::Publish(x) => x.run(metadata, &stdout, &stderr),
        Subcommand::Exec(x) => x.run(metadata, &stdout, &stderr),
        Subcommand::Create(x) => x.run(metadata, &stdout, &stderr),
    }
    .err();

    if let Some(err) = err {
        err.print(&stderr).unwrap();
    }

    stderr.flush().unwrap();
    stdout.flush().unwrap();
}
