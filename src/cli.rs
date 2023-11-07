use clap::Parser;

#[derive(Parser)]
#[clap(version, author)]
pub struct Opts {
    /// Input source file path.
    pub input: String,
    /// Output object file path.
    #[clap(short)]
    pub output: Option<String>,
}

pub fn get_opts() -> Opts {
    Opts::parse()
}
