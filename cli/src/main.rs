#[cfg(feature = "flame_it")]
#[macro_use]
extern crate flamer;

mod config;
mod time_input;

mod start;
mod summary;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "augr", about, author)]
struct Opt {
    /// Use the config file at the specified path. Defaults to `$XDG_CONFIG_HOME/augr/config.toml`.
    #[structopt(long = "config")]
    config: Option<PathBuf>,

    /// Print out where the config file was looked for
    #[structopt(long = "print-config-path")]
    print_config_path: bool,

    /// Print out what config options were used
    #[structopt(long = "print-config")]
    print_config: bool,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Add an event to the timesheet; start defaults to the current time
    #[structopt(no_version, name = "start")]
    Start(start::StartCmd),

    /// Shows a table of tracked time; defaults to only showing time tracked today
    #[structopt(no_version, name = "summary")]
    Summary(summary::SummaryCmd),
    // /// Show an ascii art chart of tracked time
    // #[structopt(no_version, name = "chart")]
    // Chart(chart::Cmd),

    // /// Get a list of all the different tags that have been used.
    // #[structopt(no_version, name = "tags")]
    // Tags(tags::TagsCmd),

    // /// Add tags to an existing event
    // #[structopt(no_version, name = "tag")]
    // Tag(tag::Cmd),

    // /// Change when an event started
    // #[structopt(no_version, name = "set-start")]
    // SetStart(set_start::Cmd),

    // /// Import data from version 0.1 of augr
    // #[structopt(no_version, name = "import")]
    // Import(import::ImportCmd),
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    // Load config
    let user_passed_in_config_flag = opt.config.is_some();
    let conf_file = match opt.config {
        Some(config_path) => config_path,
        None => {
            let proj_dirs = config::project_directories();
            proj_dirs.config_dir().join("config.toml")
        }
    };

    if opt.print_config_path {
        println!("Config path: {:?}\n", conf_file);
    }
    let config = match config::load_config(&conf_file) {
        Ok(config) => config,
        Err(e) if user_passed_in_config_flag => return Err(e),
        Err(e) => match e.root_cause().downcast_ref::<std::io::Error>() {
            Some(io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
                config::Config::default()
            }
            _ => return Err(e),
        },
    };

    if opt.print_config {
        println!(
            "sync_folder = {:?}\ndevice_id = {:?}",
            config.sync_folder, config.device_id
        );
    }

    if opt.print_config_path || opt.print_config {
        return Ok(());
    }

    // Run command
    match opt.cmd.unwrap_or_default() {
        Command::Start(subcmd) => subcmd.exec(&config)?,
        Command::Summary(subcmd) => subcmd.exec(&config)?,
    };

    Ok(())
}

fn format_duration(duration: chrono::Duration) -> String {
    let hours = duration.num_hours();
    let mins = duration.num_minutes() - (hours * 60);
    if hours < 1 {
        format!("{}m", mins)
    } else {
        format!("{}h {}m", hours, mins)
    }
}

impl Default for Command {
    fn default() -> Self {
        Command::Summary(summary::SummaryCmd::default())
    }
}
