use clap::Parser;

#[derive(Parser)]
#[clap(version = "0.1", author = "Dermot Haughey <dermot.thomas.haughey@gmail.com>")]
pub struct Config {
    #[clap(short, long, default_value = "tasks.toml")]
    pub filename: String,
}