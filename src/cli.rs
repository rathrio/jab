use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub blocks: Vec<String>,

    #[clap(long)]
    pub raw: bool,

    #[clap(long)]
    pub full: bool,

    /// Edit BRF file with text editor
    #[clap(short, long)]
    pub edit: bool,

    /// Open BRF directory with default file browser
    #[clap(long)]
    pub brf: bool,

    /// Remove the provided blocks
    #[clap(short, long)]
    pub remove: bool,

    /// Add blocks to a specific date
    #[clap(short, long)]
    pub day: Option<String>,

    /// Add comment
    #[clap(short, long)]
    pub comment: Option<String>,

    /// Remove comment
    #[clap(long)]
    pub clear_comment: bool,

    /// Add blocks to yesterday
    #[clap(short, long)]
    pub yesterday: bool,

    #[clap(short, long)]
    pub month: Option<String>,

    /// Add blocks to previous month
    #[clap(short, long)]
    pub previous: bool,

    /// Add blocks to next month
    #[clap(short, long)]
    pub next: bool,

    /// Simulate the changes and don't write them to the BRF file
    #[clap(long)]
    pub dry_run: bool,
}
