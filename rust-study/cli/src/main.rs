// cli csv -i input.csv -o output json —header -d ','
use clap::{command, parser};
#[derive(Debug, PartialEq)]
#[command (name= "cli", version, author, about, long_about = None) ]
struct Opts {
    #[commamd(subcommand)]
    cmd: Subcommand,

}

enum Subcommand {
    #[command(name="csv", about = "show CSV, or Convert CSV file to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)] // "output.json".into()
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn main() {

    println!("Hello, world!");
}
