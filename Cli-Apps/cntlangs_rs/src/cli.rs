use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "cntlangs")]
#[command(author = "Danyiel Colin <amaniel2718@protonmail.com>")]
#[command(version = "1.0")]
#[command(about = "Gives info about the files by extension", long_about = None)]
pub struct Arguments {
    /// the path where to start classifying
    #[arg(default_value = "PWD")]
    pub path: Option<String>,

    /// sort by line, file or size
    #[arg(short, long, default_value = "line")]
    sort: String,

    /// show size in plain bytes
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    bytes: bool,

    /// set if normal or inverse order
    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    inverse_order: bool,
    // since_date: bool
    // trash_files: bool
}
